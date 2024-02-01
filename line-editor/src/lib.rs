use std::{error::Error, io::{self, Write}};
use std::fs;
use std::io::{BufReader, BufRead};
use std::io::Stdin;
use std::fs::File;
use std::process;

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

pub fn read_nth_line(file_path: &str, n: usize) -> Result<String, Box<dyn Error>> {
    let file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            fs::File::create(file_path)?;
            fs::File::open(file_path)?
        }
    };

    let reader: BufReader<fs::File> = BufReader::new(file);

    reader.lines()
        .nth(n - 1) 
        .ok_or_else(|| -> Box<dyn Error> {From::from("line not found")})?
        .map_err(From::from)
}

pub fn read_lines_between_indices(file_path: &str, n1: usize, n2: usize) -> Result<String, Box<dyn Error>> {
    let file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            fs::File::create(file_path)?;
            fs::File::open(file_path)?
        }
    };

    if n1 > n2 {
        return Err("invalid range".to_string().into()).into();
    }

    let reader: BufReader<fs::File> = BufReader::new(file);
    let mut lines_string = String::new();

    for (line_number, line) in reader.lines().enumerate() {
        if line_number >= n1 - 1 && line_number < n2 {
            if line_number != n2-1 {            
                if let Ok(line) = line {
                    lines_string.push_str(&line);
                    lines_string.push('\n');
                }
            } else {
                if let Ok(line) = line {
                    lines_string.push_str(&line);
                }
            }
        }
    }

    Ok(lines_string)
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

fn write(file_path: &str, contents: &Vec<String>) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    for line in contents {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

pub fn insert_at_index(index: usize, buf_vec: Vec<String>, file_path: &str, contents: &mut Vec<String>) {
    contents.splice(index..index, buf_vec.into_iter());
    if let Err(e) = write(&file_path, contents) {
        eprintln!("write error: {e}");
        process::exit(1);
    };
}

#[derive(Debug)]
pub enum ErrorType {
    TypeErr,
    PropogatedErr(Box<dyn Error>),
    VecRangeErr,
    WriteErr(Box<dyn Error>),
    FileEmpty,
    CmdErr
}

pub fn command_handler(input: String, stdin: &Stdin, file_path: &str, contents: &mut Vec<String>) -> Result<Option<&'static str>, ErrorType> {
    let args = input
            .split_whitespace()
            .collect::<Vec<&str>>();

    let mut args_iter = args.into_iter();

    match args_iter.next() {
        Some("i") => {
            let mut buf_vec: Vec<String> = Vec::new();
            if let Some(index) = args_iter.next() {
                let n = match index.parse::<usize>() {
                    Ok(x) => x - 1,
                    Err(_) => return Err(ErrorType::TypeErr),
                };

                if n > contents.len() {
                    return Err(ErrorType::VecRangeErr);
                }

                input_mode(&stdin, &mut buf_vec);
                insert_at_index(n, buf_vec, &file_path, contents);
            } else {
                input_mode(&stdin, &mut buf_vec);
                insert_at_index(0, buf_vec, &file_path, contents);
            }
            Ok(None)
        },
        Some("p") => {
            if let Some(first_index) = args_iter.next() {
                if let Some(second_index) = args_iter.next() {
                    let n1 = match first_index.parse::<usize>() {
                        Ok(x) => x,
                        Err(_) => {
                            return Err(ErrorType::TypeErr);
                        }
                    };
                    let n2 = match second_index.parse::<usize>() {
                        Ok(x) => x,
                        Err(_) => {
                            return Err(ErrorType::TypeErr);
                        }
                    };
                    let lines = match read_lines_between_indices(file_path, n1, n2) {
                        Ok(x) => x,
                        Err(e) => {
                            return Err(ErrorType::PropogatedErr(e));
                        }
                    };
                    println!("{lines}");
                    Ok(None)
                } else {
                    let n = match first_index.parse::<usize>() {
                        Ok(x) => x,
                        Err(_) => {
                            return Err(ErrorType::TypeErr);
                        }
                    };
                    let line = match read_nth_line(&file_path, n) {
                        Ok(x) => x,
                        Err(e) => {
                            return Err(ErrorType::PropogatedErr(e));
                        }
                    };
                    println!("{line}");
                    Ok(None)
                }
            } else {
                let line = match read_nth_line(&file_path, 1) {
                    Ok(x) => x,
                    Err(e) => {
                        return Err(ErrorType::PropogatedErr(e));
                    }
                };
                println!("{line}");
                Ok(None)
            }
        },
        Some("d") => {
            if let Some(first_index) = args_iter.next() {
                if let Some(second_index) = args_iter.next() {
                    let n1 = match first_index.parse::<usize>() {
                        Ok(x) => x,
                        Err(_) => {
                            return Err(ErrorType::TypeErr);
                        }
                    };
                    let n2 = match second_index.parse::<usize>() {
                        Ok(x) => x,
                        Err(_) => {
                            return Err(ErrorType::TypeErr);
                        }
                    };

                    let mut contents = read_into_vec(&file_path).unwrap();
                    if n1 < n2 && n2 <= contents.len() {
                        contents.drain(n1-1..n2-1);
                    } else {
                        return Err(ErrorType::VecRangeErr);
                    }

                    if let Err(e) = write(&file_path, &contents) {
                        return Err(ErrorType::WriteErr(Box::new(e)));
                    };

                    Ok(None)
                } else {
                    let n = match first_index.parse::<usize>() {
                        Ok(x) => x,
                        Err(_) => {
                            return Err(ErrorType::TypeErr);
                        }
                    };

                    let mut contents = read_into_vec(&file_path).unwrap();
                    if n <= contents.len() {
                        contents.remove(n-1);
                    } else {
                        return Err(ErrorType::VecRangeErr);
                    }

                    if let Err(e) = write(&file_path, &contents) {
                        return Err(ErrorType::WriteErr(Box::new(e)));
                    };

                    Ok(None)
                }
            } else {
                let mut contents = read_into_vec(&file_path).unwrap();
                if contents.len() != 0 {
                    contents.remove(0);
                } else {
                    return Err(ErrorType::FileEmpty);
                }

                if let Err(e) = write(&file_path, &contents) {
                    return Err(ErrorType::WriteErr(Box::new(e)));
                };

                Ok(None)
            }
        },
        Some("exit") => return Ok(Some("kill")),
        _ => return Err(ErrorType::CmdErr),
    }
}

pub fn execute(file_path: &str, contents: &mut Vec<String>, stdin: &Stdin) {
    loop {
        let input = process_input().unwrap_or_else(|e| {
            eprintln!("{e}");
            process::exit(1);
        });

        match command_handler(input, stdin, file_path, contents) {
            Ok(None) => {},
            Ok(Some(x)) => if x == "kill" {process::exit(0)},
            Err(e) => println!("{:?}", e),
        }
    }
}