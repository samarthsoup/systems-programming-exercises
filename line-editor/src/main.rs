use std::io;
use std::process;
use std::env;

fn main() {
    let stdin = io::stdin();
    let mut buf_vec: Vec<String> = Vec::new();

    let file_path = line_editor::build(env::args()).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    let mut contents = line_editor::read_into_vec(&file_path).unwrap_or_else(|e|  {
        eprintln!("{e}");
        process::exit(1);
    });

    println!("lines: {}", contents.len());

    loop {
        let input = line_editor::process_input().unwrap_or_else(|e| {
            eprintln!("{e}");
            process::exit(1);
        });

        let args = input
            .split_whitespace()
            .collect::<Vec<&str>>();

        match args[0] {
            "i" => {
                line_editor::input_mode(&stdin, &mut buf_vec);
                let index = match args[1].parse::<usize>() {
                    Ok(x) => x - 1,
                    Err(_) => continue,
                };
                if index <= contents.len() {
                    contents.splice(index..index, buf_vec.iter().cloned());
                    buf_vec.clear();
                    if let Err(e) = line_editor::write(&file_path, &mut contents) {
                        eprintln!("write error: {e}");
                        process::exit(1);
                    };
                } else {
                    println!("insert index is out of bounds");
                }
            },
            "exit" => break,
            _ => eprintln!("unrecognised command"),
        }
    }
}
