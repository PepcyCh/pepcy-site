mod blog;
mod single;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 1 {
        blog::gen_mds("../markdowns/algo", "../generated/algo");
        println!("algo - done!");

        blog::gen_mds("../markdowns/note", "../generated/note");
        println!("note - done!");

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
        } else {
            println!("unknown command line args");
        }
    } else {
        println!("unknown command line args");
    }
}
