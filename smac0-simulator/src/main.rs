#![allow(non_camel_case_types)]

use std::fs::File;
use std::io::{self, Write};
use std::process;

struct int_char(char);

impl int_char {
    fn new(c: char) -> Option<Self> {
        if c.is_digit(10){
            Some(int_char(c))
        } else {
            None
        }
    }
}

fn process_input() -> Result<String, io::Error> {
    print!("? ");
    io::stdout().flush()?;

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.to_string()), 
        Err(e) => Err(e),
    }
}

fn main() {
    let mut memory: [[int_char; 6]; 1000];
    let mut registers: [[int_char; 6]; 4];
    let mut program_counter: [int_char; 3];
    let mut condition_code: int_char;
    let mut last_valid_addr: [int_char; 3];

    loop {
        let input = process_input().unwrap_or_else(|e| {
            eprintln!("{e}");
            process::exit(1);
        });

        let mut args = input.split_whitespace().into_iter();

        match args.next() {
            Some("load") => {
                if let Some(file_name) = args.next() {
                    let file = File::open(file_name).unwrap();
                    
                }
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
