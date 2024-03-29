#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::fs;
use std::io::{BufRead, BufReader};
use std::error::Error;

pub struct symbol_table {
    symbol: String,
    addr: usize,
    used: bool,
    defined: bool
}

pub enum operand_type {
    symbol,
    constant
}

pub struct intermediate_code_table {
    addr: usize,
    opcode: usize,
    register_operand: usize,
    operand_type: operand_type,
    value: usize
}

pub struct error_table {
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
    let mne = ["STOP", "ADD", "SUB", "MULT", "DIV", "MOVER", "MOVEM", "COMP", "BC", "READ", "PRINT", "END"].map(|x| x.to_string());
    let reg = ["AREG", "BREG", "CREG", "DREG"].map(|x| x.to_string());
    let cc = ["LT", "LE", "EQ", "GT", "GE", "ANY"].map(|x| x.to_string());
    let err = ["used but not defined", "invalid opcode", "wrong statement format", "no start label", "no end label"].map(|x| x.to_string());

    let mut symbol_table: Vec<symbol_table> = Vec::new();
    let mut intermediate_code_table: Vec<intermediate_code_table> = Vec::new();
    let mut error_table: Vec<error_table> = Vec::new();

    let last_index = lines.len();

    let start_index = match lines.iter().position(|r| r == "START") {
        Some(x) => x,
        None => {
            error_table.push(error_table{err_line_no: 0, err_index: 3});
            0
        }
    };

    for i in start_index..=last_index {
        
    }


}