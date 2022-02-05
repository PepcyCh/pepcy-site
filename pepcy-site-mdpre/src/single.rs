use std::{io::{self, Read, Write}, fs};

lazy_static::lazy_static! {
    static ref MARKDOWN_OPTS: comrak::ComrakOptions = {
        let extension = comrak::ComrakExtensionOptions {
            table: true,
            ..Default::default()
        };
        let render = comrak::ComrakRenderOptions {
            unsafe_: true,
            ..Default::default()
        };
        comrak::ComrakOptions { extension, parse: Default::default(), render }
    };
}

pub fn gen_md(src_dir: &str, dst_dir: &str) {
    let file = fs::File::open(src_dir).unwrap();
    let mut reader = io::BufReader::new(file);
    let mut markdown_str = String::new();
    reader.read_to_string(&mut markdown_str).unwrap();

    let html_str = comrak::markdown_to_html(&markdown_str, &MARKDOWN_OPTS);

    let file = fs::File::create(dst_dir).unwrap();
    let mut writer = io::BufWriter::new(file);
    writer.write_all(html_str.as_bytes()).unwrap();
}