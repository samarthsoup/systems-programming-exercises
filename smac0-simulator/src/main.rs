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

fn parse_file(contents: String, memory: &mut [usize; 1000], mut program_counter: usize) {
    let lines: Vec<&str> = contents.lines().collect();

    for line in lines {
        if line.starts_with("-1") {
            program_counter = line[3..=5].parse::<usize>().unwrap();
        } else {
            let addr = &line[..=2].parse::<usize>().unwrap();
            memory[*addr] = line[4..].parse::<usize>().unwrap();
        }
    }
}

fn load_program(filename: &str, memory: &mut [usize; 1000], program_counter: usize) {
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    parse_file(contents, memory, program_counter);

}

fn main() {
    let mut memory: [usize; 1000] = todo!();
    let mut registers: [usize; 4];
    let mut program_counter: usize;
    let mut condition_code: usize;
    let mut last_valid_addr: usize;

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

                load_program(filename, &mut memory, program_counter);
            },
            Some("print") => {},
            Some("accept") => {},
            Some("run") => {},
            Some("trace") => {},
            Some("quit") => break,
            _ => continue,
        }
    }
    
}
