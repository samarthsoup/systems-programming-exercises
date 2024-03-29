#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::fs;
use std::io::{BufRead, BufReader};
use std::error::Error;

struct intermediate_code_table {
    addr: usize,
    opcode: usize,
    register_operand: usize,
    memory_operand: usize,
    defined: bool
}

struct error_table {
    err_line_no: usize,
    err_index: usize
}

pub fn build(mut args: impl Iterator<Item = String>) -> Result<String, &'static str> {
    args.next();

    let file_path =  match args.next() {
        Some(arg) => arg,
        None => return Err("didn't get a file path"),
    };

    Ok(file_path)
}

pub fn read_into_vec(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            fs::File::create(file_path)?;
            fs::File::open(file_path)?
        }
    };

    let reader: BufReader<fs::File> = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

pub fn assemble(lines: Vec<String>) {
    let mne: Vec<String> = ["ADD", "SUB", "MULT", "DIV", "MOVER", "MOVEM", "COMP", "BC", "READ", "PRINT"].map(|x| x.to_string()).to_vec();
    let reg = ["AREG", "BREG", "CREG", "DREG"].map(|x| x.to_string());
    let err = ["used but not defined", "invalid opcode", "wrong statement format", "no start label", "no end label"].map(|x| x.to_string());

    let mut intermediate_code_table: Vec<intermediate_code_table> = Vec::new();
    let mut error_table: Vec<error_table> = Vec::new();

    let last_index = match lines.iter().position(|r| r == "END") {
        Some(x) => x,
        None => {
            error_table.push(error_table{err_line_no: 0, err_index: 4});
            0
        }
    };

    let start_index = match lines.iter().position(|r| r == "START") {
        Some(x) => x,
        None => {
            error_table.push(error_table{err_line_no: 0, err_index: 3});
            0
        }
    };

    if start_index > last_index {
        error_table.push(error_table{err_line_no: 0, err_index: 3});
    }

    let start_tokens: Vec<&str> = lines[start_index].split_whitespace().collect();
    let mut addr: usize = start_tokens[1].parse::<usize>().unwrap();

    for i in start_index+1..last_index {
        let tokens: Vec<&str> = lines[i].split_whitespace().collect();

        let mne_index = match mne.iter().position(|r| r == tokens[0]){
            Some(x) => x,
            None => {
                error_table.push(error_table{err_line_no: i, err_index: 1});
                10 //10th index doesnt exist in the mne table, so its a placeholder index to indicate that it is the wrong opcode
            }
        };
        let reg_index = match reg.iter().position(|r| r == tokens[1]){
            Some(x) => x,
            None => {
                error_table.push(error_table{err_line_no: i, err_index: 2});
                4
            }
        };
        let mem_index = tokens[2].parse::<usize>().unwrap();

        intermediate_code_table.push(intermediate_code_table { addr, opcode: mne_index, register_operand: reg_index, memory_operand: mem_index, defined: true });

        addr+=1;
    }

    for x in error_table {
        println!("err({}): {}", x.err_line_no, err[x.err_index]);
    }

    let mut machine_code: Vec<String> = Vec::new();

    if err.len() == 0 {
        for i in 0..intermediate_code_table.len() {
            machine_code[i] = format!("{} {}{}{}", intermediate_code_table[i].addr, 
                intermediate_code_table[i].opcode, 
                intermediate_code_table[i].register_operand, 
                intermediate_code_table[i].memory_operand);
        }
    }
}