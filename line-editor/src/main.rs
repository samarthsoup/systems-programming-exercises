use std::io;
use std::process;
use std::env;

fn main() {
    let stdin = io::stdin();
    let buf_vec: Vec<String> = Vec::new();

    let file_path = line_editor::build(env::args()).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    let mut contents = line_editor::read_into_vec(&file_path).unwrap_or_else(|e|  {
        eprintln!("{e}");
        process::exit(1);
    });

    println!("lines: {}", contents.len());

    line_editor::execute(buf_vec, &file_path, &mut contents, &stdin);
}
