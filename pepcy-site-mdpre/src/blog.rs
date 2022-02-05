use std::{
    collections::{BTreeMap, HashMap},
    fs,
    io::{self, Read, Write},
};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
struct BlogHeader {
    title: String,
    url: String,
    tags: Vec<String>,
    part: String,
    create_time: i64,
    last_modified: i64,
}

#[derive(Debug, serde::Serialize)]
struct Blog<'a> {
    title: &'a str,
    tags: &'a [String],
    create_time: i64,
    last_modified: i64,
    toc: String,
    html: String,
}

#[derive(Debug, serde::Serialize)]
struct Tag<'a> {
    name: &'a str,
    count: usize,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct BlogCacheItem {
    header: BlogHeader,
    id: u64,
    last_modified: i64,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct TagCacheItem {
    id: u64,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct Cache {
    blogs: HashMap<String, BlogCacheItem>,
    tags: HashMap<String, TagCacheItem>,
    next_blog_id: u64,
    next_tag_id: u64,
}

fn parse_header(headre_str: &str) -> BlogHeader {
    let mut header = BlogHeader::default();

    for info in headre_str
        .replace("---", "")
        .trim()
        .split('\n')
        .map(|str| str.trim())
    {
        let (field, data) = info.split_once(':').unwrap();
        match field {
            "title" => {
                header.title = data.trim().trim_matches('\'').to_owned();
            }
            "url" => {
                header.url = data.trim().to_owned();
            }
            "tags" => {
                header.tags = data
                    .trim()
                    .trim_matches(|c| c == '[' || c == ']')
                    .split(',')
                    .map(|str| str.trim().to_owned())
                    .filter(|str| !str.is_empty())
                    .collect();
            }
            "date" => {
                header.create_time =
                    chrono::NaiveDateTime::parse_from_str(data, "%Y-%m-%d %H:%M:%S")
                        .unwrap()
                        .timestamp();
            }
            _ => {}
        }
    }

    header
}

lazy_static::lazy_static! {
    static ref MARKDOWN_OPTS: comrak::ComrakOptions = {
        let extension = comrak::ComrakExtensionOptions {
            table: true,
            header_ids: Some("".to_owned()),
            ..Default::default()
        };
        let render = comrak::ComrakRenderOptions {
            unsafe_: true,
            ..Default::default()
        };
        comrak::ComrakOptions { extension, parse: Default::default(), render }
    };
    static ref KATEX_OPTS: katex::Opts = katex::Opts::builder().display_mode(true).build().unwrap();
    static ref TIMEZONE: chrono::FixedOffset = chrono::FixedOffset::east(8 * 3600);
}

struct MathBlockReplacer;

impl regex::Replacer for MathBlockReplacer {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        if dst.chars().filter(|ch| *ch == '`').count() % 2 == 1 {
            dst.push_str(caps.get(0).unwrap().as_str());
            return;
        }
        let content = caps.get(1).unwrap().as_str();
        let math_html = match katex::render_with_opts(content, KATEX_OPTS.as_ref()) {
            Ok(html) => html,
            Err(err) => {
                println!(
                    "[WARN] math block \"{}\" failed to be processed, err: {}",
                    content,
                    err.to_string()
                );
                content.to_owned()
            }
        };
        dst.push_str(format!("<p class='katex-block'>{}</p>", math_html).as_str());
    }
}

struct InlineMathReplacer;

impl regex::Replacer for InlineMathReplacer {
    fn replace_append(&mut self, caps: &lazy_regex::Captures<'_>, dst: &mut String) {
        if dst.chars().filter(|ch| *ch == '`').count() % 2 == 1 {
            dst.push_str(caps.get(0).unwrap().as_str());
            return;
        }
        let content = caps.get(1).unwrap().as_str();
        let math_html = match katex::render(content) {
            Ok(html) => html,
            Err(err) => {
                println!(
                    "[WARN] inline math \"{}\" failed to be processed, err: {}",
                    content,
                    err.to_string()
                );
                content.to_owned()
            }
        };
        dst.push_str(&math_html);
    }
}

struct FilenameReplacer;

impl regex::Replacer for FilenameReplacer {
    fn replace_append(&mut self, _caps: &lazy_regex::Captures<'_>, dst: &mut String) {
        dst.push('-');
    }
}

fn parse_markdown(markdown_str: &str) -> (String, String) {
    let adapter = comrak::plugins::syntect::SyntectAdapter::new("base16-ocean.dark");
    let mut plugins = comrak::ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let html_str = markdown_str;

    let math_block_re = lazy_regex::regex!(r#"\$\$(([^\$`]|\n)+)\$\$"#);
    let math_block_replacer = MathBlockReplacer;
    let html_str = math_block_re.replace_all(&html_str, math_block_replacer);

    let inline_math_re = lazy_regex::regex!(r#"\$([^\$`]+)\$"#);
    let inline_math_replacer = InlineMathReplacer;
    let html_str = inline_math_re.replace_all(&html_str, inline_math_replacer);

    let html_str =
        comrak::markdown_to_html_with_plugins(html_str.as_ref(), &MARKDOWN_OPTS, &plugins);

    let part_re = lazy_regex::regex!(r#"<!--+\s*more\s*--+>"#);
    let part = if let Some(cap) = part_re.captures(&html_str) {
        let mat = cap.get(0).unwrap();
        html_str.split_at(mat.start()).0.to_owned()
    } else {
        "".to_owned()
    };

    (html_str.to_string(), part)
}

fn gen_lists(blogs: &[BlogHeader], num_per_page: u32, dst_dir: &str) -> usize {
    let mut page_count = 0;

    let total_page_count = (blogs.len() as u32 + num_per_page - 1) / num_per_page;

    for blogs in blogs.chunks(num_per_page as usize) {
        page_count += 1;
        let content_json = serde_json::json!({
            "page_count": total_page_count,
            "articles": blogs
        });
        let content = content_json.to_string();
        let file = fs::File::create(format!("{}/_page{}.json", dst_dir, page_count)).unwrap();
        let mut writer = io::BufWriter::new(file);
        writer.write_all(content.as_bytes()).unwrap();
    }

    page_count
}

fn gen_tags(sets: &BTreeMap<&String, Vec<(&str, &str)>>, dst_dir: &str) {
    for (name, articles) in sets {
        let content = serde_json::to_string(articles).unwrap();
        let file = fs::File::create(format!("{}/_tag_{}.json", dst_dir, name)).unwrap();
        let mut writer = io::BufWriter::new(file);
        writer.write_all(content.as_bytes()).unwrap();

        println!("tag '{}' generated", name);
    }

    let sets = sets
        .iter()
        .map(|set| Tag {
            name: set.0.as_str(),
            count: set.1.len(),
        })
        .collect::<Vec<_>>();
    let content = serde_json::to_string(&sets).unwrap();
    let file = fs::File::create(format!("{}/_tag.json", dst_dir)).unwrap();
    let mut writer = io::BufWriter::new(file);
    writer.write_all(content.as_bytes()).unwrap();
}

fn gen_toc_html(html_str: &str) -> String {
    let math_block_re = lazy_regex::regex!(r#"<h(\d)><a href="\#(.*)" aria.*</a>(.*)</h(\d)>"#);

    let mut toc_html = "".to_owned();
    let mut last_level = 0;
    for (level, id, content) in math_block_re.captures_iter(html_str).filter_map(|cap| {
        let level: i32 = std::str::FromStr::from_str(cap.get(1).unwrap().as_str()).unwrap();
        if level >= 2 && level <= 4 {
            let id = cap.get(2).unwrap().as_str();
            let content = cap.get(3).unwrap().as_str();
            Some((level, id, content))
        } else {
            None
        }
    }) {
        if level > last_level {
            toc_html.push_str("<ol>");
        } else if level < last_level {
            toc_html.push_str("</ol>".repeat((last_level - level) as usize).as_str());
        }
        toc_html.push_str(format!("<li><a href=\"#{}\">{}</a></li>", id, content).as_str());
        last_level = level;
    }

    toc_html
}

fn read_cache(dst_dir: &str) -> Cache {
    if let Ok(file) = fs::File::open(format!("{}/_cache.json", dst_dir)) {
        let mut reader = io::BufReader::new(file);
        let mut cache = String::new();
        reader.read_to_string(&mut cache).unwrap();
        serde_json::from_str(&cache).unwrap()
    } else {
        Cache::default()
    }
}

fn write_cache(cache: Cache, dst_dir: &str) {
    let file = fs::File::create(format!("{}/_cache.json", dst_dir)).unwrap();
    let mut writer = io::BufWriter::new(file);
    let cache = serde_json::to_string(&cache).unwrap();
    writer.write_all(cache.as_bytes()).unwrap();
}

pub fn gen_mds(src_dir: &str, dst_dir: &str) {
    let mut cache = read_cache(dst_dir);

    let mut blogs = vec![];

    for f in fs::read_dir(src_dir)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .filter(|path| {
            path.as_ref()
                .map_or(false, |path| path.extension().unwrap() == "md")
        })
        .map(|path| path.unwrap())
    {
        let filename_re = lazy_regex::regex!(r#"\s+"#);
        let filename = f.file_stem().unwrap().to_str().unwrap();
        let filename_replacer = FilenameReplacer;
        let filename = filename_re
            .replace_all(filename, filename_replacer)
            .to_string();

        let file = fs::File::open(f).unwrap();
        let metadata = file.metadata().unwrap();
        let last_modified: chrono::DateTime<chrono::Utc> = metadata.modified().unwrap().into();
        let last_modified = last_modified
            .with_timezone(&*TIMEZONE)
            .naive_local()
            .timestamp();
        let need_to_process = if let Some(cached_blog) = cache.blogs.get(&filename) {
            last_modified > cached_blog.last_modified
        } else {
            true
        };

        let header = if need_to_process {
            let mut reader = io::BufReader::new(file);
            let mut content = String::new();
            reader.read_to_string(&mut content).unwrap();

            let header_end_pos = content.match_indices("---").nth(1).unwrap().0 + 3;
            let (header, content) = content.split_at(header_end_pos);

            let mut header = parse_header(header);
            let (html, part) = parse_markdown(content);
            let toc = gen_toc_html(&html);
            let blog = Blog {
                title: &header.title,
                tags: &header.tags,
                create_time: header.create_time,
                last_modified,
                toc,
                html,
            };
            header.part = part;
            header.last_modified = last_modified;
            if header.url.is_empty() {
                header.url = filename.clone();
            }
            header.url = percent_encoding::utf8_percent_encode(
                &header.url,
                percent_encoding::NON_ALPHANUMERIC,
            )
            .to_string();

            let file = fs::File::create(format!("{}/{}.json", dst_dir, filename)).unwrap();
            let content = serde_json::to_string(&blog).unwrap();
            let mut writer = io::BufWriter::new(file);
            writer.write_all(content.as_bytes()).unwrap();

            if let Some(cached_blog) = cache.blogs.get_mut(&filename) {
                cached_blog.header = header.clone();
                cached_blog.last_modified = last_modified;
            } else {
                let id = cache.next_blog_id;
                cache.next_blog_id += 1;
                let item = BlogCacheItem {
                    header: header.clone(),
                    id,
                    last_modified,
                };
                cache.blogs.insert(filename.clone(), item);
            }

            println!("article '{}' generated", filename);

            header
        } else {
            println!("article '{}' cached", filename);

            cache.blogs.get(&filename).unwrap().header.clone()
        };

        blogs.push(header);
    }
    blogs.sort_by(|a, b| b.create_time.cmp(&a.create_time));

    // title lists
    let _page_count = gen_lists(&blogs, 10, dst_dir);

    // tags
    let mut tags = BTreeMap::new();
    for blog in &blogs {
        for tag in &blog.tags {
            tags.entry(tag)
                .or_insert(vec![])
                .push((blog.title.as_str(), blog.url.as_str()));
        }
    }
    for (&name, _) in &tags {
        if !cache.tags.contains_key(name) {
            let id = cache.next_tag_id;
            cache.next_tag_id += 1;
            let item = TagCacheItem { id };
            cache.tags.insert(name.clone(), item);
        }
    }
    gen_tags(&tags, dst_dir);

    write_cache(cache, dst_dir);
}
