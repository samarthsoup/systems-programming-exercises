use std::process;
use std::env;

fn main() {
    let file_path = line_editor::build(env::args()).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    let contents = line_editor::read_into_vec(&file_path).unwrap_or_else(|e|  {
        eprintln!("{e}");
        process::exit(1);
    });

    println!("lines: {}", contents.len());

    /*loop {
        if let Err(e) = line_editor::process_input() {
            eprintln!("{e}");
            process::exit(1);
        };
    }*/
}
