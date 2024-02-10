#![allow(non_camel_case_types)]

use std::fs;
use std::io::{self, Write};
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

fn execute(memory: &mut [usize; 1000], mut program_counter: usize, last_logical_addr: usize, registers: &mut [usize; 4]) {
    while program_counter < last_logical_addr {
        println!("program_counter: {program_counter}");
        let mem_str = memory[program_counter].to_string();
        let mut condition_codes: [bool; 6] = [false; 6];
        let (mut opcode, mut register_op, mut mem_op) = (0, 0, 0);

        if mem_str.len() == 1 {
            break;
        }

        if mem_str.len() == 6 {
            (opcode, register_op, mem_op) = ((&mem_str[..=1]).parse::<usize>().unwrap(), (&mem_str[2..=2]).parse::<usize>().unwrap(), (&mem_str[3..=5]).parse::<usize>().unwrap());
        } else {
            (opcode, register_op, mem_op) = ((&mem_str[..=0]).parse::<usize>().unwrap(), (&mem_str[1..=1]).parse::<usize>().unwrap(), (&mem_str[2..=4]).parse::<usize>().unwrap());
        }

        match opcode {
            0 => break,
            1 => registers[register_op] += memory[mem_op],
            2 => registers[register_op] -= memory[mem_op],
            3 => registers[register_op] *= memory[mem_op],
            8 => registers[register_op] /= memory[mem_op],
            4 => registers[register_op] = memory[mem_op],
            5 => memory[mem_op] = registers[register_op],
            6 => {
                condition_codes[0] = registers[register_op] <  memory[mem_op];
                condition_codes[1] = registers[register_op] <=  memory[mem_op];
                condition_codes[2] = registers[register_op] ==  memory[mem_op];
                condition_codes[3] = registers[register_op] >  memory[mem_op];
                condition_codes[4] = registers[register_op] >=  memory[mem_op];
                condition_codes[5] = true;
            },
            7 => {
                if register_op == 5 || condition_codes[register_op] {
                    program_counter = mem_op;
                    continue;
                }
                
            },
            9 => {
                println!("taking input for mem block {mem_op}:");
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => Ok(input.to_string()), 
                    Err(e) => Err(e),
                };
                let input_int = input.trim().parse::<usize>().unwrap();
                memory[mem_op] = input_int;
            },
            10 => println!("{}", memory[mem_op]),
            _ => panic!("invalid opcode"),
        }
        program_counter += 1;
    }
}

fn main() {
    let mut memory: [usize; 1000] = [0; 1000];
    let mut registers: [usize; 4] = [0; 4];
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
            Some("run") => {
                execute(&mut memory, program_counter, last_logical_addr, &mut registers);
            },
            Some("trace") => {},
            Some("quit") => break,
            _ => continue,
        }
    }
    
}
