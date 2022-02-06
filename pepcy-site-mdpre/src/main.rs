mod utils;
mod blog;
mod single;
mod new;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 1 {
        blog::gen_mds("../markdowns/algo", "../generated/algo");
        println!("algo - done!");

        blog::gen_mds("../markdowns/note", "../generated/note");
        println!("note - done!");

        blog::gen_mds("../markdowns/acgn", "../generated/acgn");
        println!("acgn - done!");

        single::gen_md("../markdowns/about.md", "../generated/about.html");
        println!("about - done!");

        single::gen_md("../markdowns/resume.md", "../generated/resume.html");
        println!("resume - done!");

        single::gen_md("../markdowns/404.md", "../generated/404.html");
        println!("404 - done!");
    } else if args.len() == 2 {
        if args[1] == "clean" {
            std::fs::remove_dir_all("../generated/algo/").unwrap();
            std::fs::remove_dir_all("../generated/note/").unwrap();
            std::fs::remove_file("../generated/about.html").unwrap();
            std::fs::remove_file("../generated/resume.html").unwrap();
            std::fs::remove_file("../generated/404.html").unwrap();
        } else {
            println!("unknown command line args");
        }
    } else if args.len() == 4 {
        if args[1] == "new" {
            new::new_md(format!("../markdowns/{}", args[2]).as_str(), args[3].as_str());
        } else {
            println!("unknown command line args");
        }
    } else {
        println!("unknown command line args");
    }
}
