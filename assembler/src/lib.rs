#![warn(dead_code)]
#![warn(non_camel_case_types)]

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

pub fn assemble() {
    let mne = ["STOP", "ADD", "SUB", "MULT", "DIV", "MOVER", "MOVEM", "COMP", "BC", "READ", "PRINT", "END"].map(|x| x.to_string());
    let reg = ["AREG", "BREG", "CREG", "DREG"].map(|x| x.to_string());
    let cc = ["LT", "LE", "EQ", "GT", "GE", "ANY"].map(|x| x.to_string());
    let err = ["used but not defined", "invalid opcode", "wrong statement format"].map(|x| x.to_string());
}