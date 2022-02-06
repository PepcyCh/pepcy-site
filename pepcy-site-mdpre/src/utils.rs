lazy_static::lazy_static! {
    pub static ref MARKDOWN_OPTS: comrak::ComrakOptions = {
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
    pub static ref KATEX_OPTS: katex::Opts = katex::Opts::builder().display_mode(true).build().unwrap();
    pub static ref TIMEZONE: chrono::FixedOffset = chrono::FixedOffset::east(8 * 3600);
}

pub struct MathBlockReplacer;

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

pub struct InlineMathReplacer;

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

pub struct FilenameReplacer;

impl regex::Replacer for FilenameReplacer {
    fn replace_append(&mut self, _caps: &lazy_regex::Captures<'_>, dst: &mut String) {
        dst.push('-');
    }
}