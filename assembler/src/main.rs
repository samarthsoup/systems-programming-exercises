use std::env;
use std::process;

fn main() {
    let file_path = assembler::build(env::args()).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    let lines = assembler::read_into_vec(&file_path).unwrap_or_else(|e|  {
        eprintln!("{e}");
        process::exit(1);
    });

    assembler::assemble(lines);
}
