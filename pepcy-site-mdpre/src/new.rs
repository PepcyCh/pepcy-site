use std::{io::{self, Write}, fs};

use crate::utils::*;

pub fn new_md(md_dir: &str, name: &str) {
    let filename_re = lazy_regex::regex!(r#"\s+"#);
    let filename_replacer = FilenameReplacer;
    let filename = filename_re.replace_all(name, filename_replacer).to_string();

    let time = chrono::Utc::now().with_timezone(&*TIMEZONE).naive_local();
    let time_str = time.format("%Y-%m-%d %H:%M:%S").to_string();

    let start_str = format!("---\ntitle: {}\ndate: {}\n---\n\n", name, time_str);
    let file = fs::File::create(format!("{}/{}.md", md_dir, filename)).unwrap();
    let mut writer = io::BufWriter::new(file);
    writer.write_all(start_str.as_bytes()).unwrap();

    println!("{}/{}.md is created", md_dir, filename);
}