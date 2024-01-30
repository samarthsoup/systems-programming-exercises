use std::process;
use std::env;

fn main() {
    let mut input_mode = false;

    let file_path = line_editor::build(env::args()).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    let contents = line_editor::read_into_vec(&file_path).unwrap_or_else(|e|  {
        eprintln!("{e}");
        process::exit(1);
    });

    println!("lines: {}", contents.len());

    loop {
        let input = line_editor::process_input().unwrap_or_else(|e| {
            eprintln!("{e}");
            process::exit(1);
        });

        if !input_mode{
            let args = input
                .split_whitespace()
                .collect::<Vec<&str>>();

            match args[0] {
                "i" => {
                    input_mode = true;
                },
                _ => eprintln!("unrecognised command"),
            }
        }
    }
}
