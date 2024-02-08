#![allow(non_camel_case_types)]

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::process;

fn process_input() -> Result<String, io::Error> {
    print!("? ");
    io::stdout().flush()?;

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.to_string()), 
        Err(e) => Err(e),
    }
}

fn parse_file(contents: String, memory: &mut [usize; 1000]) -> (usize, usize) {
    let lines: Vec<&str> = contents.lines().collect();
    let mut last_logical_addr: usize = 0;
    let mut program_counter: usize = 0;

    for line in lines {
        if line.starts_with("-1") {
            program_counter = line[3..=5].parse::<usize>().unwrap();
        } else {
            let addr = &line[..=2].parse::<usize>().unwrap();
            last_logical_addr = *addr;
            memory[*addr] = line[4..].parse::<usize>().unwrap();
        }
    }

    (program_counter, last_logical_addr)
}

fn load_program(filename: &str, memory: &mut [usize; 1000]) -> (usize, usize) {
    let contents = fs::read_to_string(filename)
        .expect("should have been able to read the file");

    parse_file(contents, memory)
}

fn print_loaded_program(memory: [usize; 1000], program_counter: usize, last_logical_addr: usize) {
    for i in program_counter..=last_logical_addr {
        println!("{}", &memory[i]);
    }
}

fn main() {
    let mut memory: [usize; 1000] = [0; 1000];
    let mut registers: [usize; 4];
    let mut program_counter: usize = 0;
    let mut condition_code: usize = 0;
    let mut last_logical_addr: usize = 0;

    loop {
        let input = process_input().unwrap_or_else(|e| {
            eprintln!("{e}");
            process::exit(1);
        });

        let mut args = input.split_whitespace().into_iter();

        match args.next() {
            Some("load") => {
                let filename = match args.next() {
                    Some(x) => x,
                    None => continue,
                };

                (program_counter, last_logical_addr) = load_program(filename, &mut memory);

            },
            Some("print") => {
                print_loaded_program(memory, program_counter, last_logical_addr);
            },
            Some("accept") => {},
            Some("run") => {},
            Some("trace") => {},
            Some("quit") => break,
            _ => continue,
        }
    }
    
}
