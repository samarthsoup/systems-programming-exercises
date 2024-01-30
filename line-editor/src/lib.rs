use std::{error::Error, io::{self, Write}};
use std::fs;
use std::io::{BufReader, BufRead};
use std::io::Stdin;
use std::fs::File;

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
            fs::File::open(file_path).unwrap()
        }
    };

    let reader: BufReader<fs::File> = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

pub fn process_input() -> Result<String, io::Error> {
    print!("? ");
    io::stdout().flush()?;

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.to_string()), 
        Err(e) => Err(e),
    }
}

pub fn input_mode(stdin: &Stdin, buf_vec: &mut Vec<String>) {
    let mut input_mode = true;
    println!("::entering-input-mode");
    while input_mode {
        let mut data = String::new();
        stdin.read_line(&mut data).unwrap();

        if data.trim() == "." {
            input_mode = false;
            println!("::exiting-input-mode");
        } else {
            buf_vec.push(data.trim_end_matches("\r\n").to_string());
        }
    }
}

pub fn write(file_path: &str, lines: &Vec<String>) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}