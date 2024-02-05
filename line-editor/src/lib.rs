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

fn process_input() -> Result<String, io::Error> {
    print!("? ");
    io::stdout().flush()?;

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.to_string()), 
        Err(e) => Err(e),
    }
}

fn input_mode(stdin: &Stdin, buf_vec: &mut Vec<String>) {
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

fn insert_at_index(
    index: usize, 
    buf_vec: Vec<String>, 
    contents: &mut Vec<String>
) {
    contents.splice(index..index, buf_vec.into_iter());
}

fn append_to_end(
    buf_vec: Vec<String>, 
    contents: &mut Vec<String>
) {
    contents.extend(buf_vec.into_iter());
}

#[derive(Debug)]
pub enum ErrorType {
    TypeErr,
    PropogatedErr(Box<dyn Error>),
    RangeError,
    WriteErr(Box<dyn Error>),
    FileEmpty,
    CmdErr,
    ArgCountErr
}

struct ReturnType {
    msg: String,
}

impl ReturnType {
    fn new() -> ReturnType{
        ReturnType {
            msg: String::new()
        }
    }
}

fn insert(
    args_iter: &mut dyn Iterator<Item = &str>, 
    contents: &mut Vec<String>, 
    stdin: &Stdin
) -> Result<Option<ReturnType>, ErrorType> {
    let mut buf_vec: Vec<String> = Vec::new();
    if let Some(index) = args_iter.next() {
        let n = match index.parse::<usize>() {
            Ok(x) => x - 1,
            Err(_) => return Err(ErrorType::TypeErr),
        };

        if n >= contents.len() {
            return Err(ErrorType::RangeError);
        }

        input_mode(&stdin, &mut buf_vec);
        insert_at_index(n, buf_vec, contents);
    } else {
        input_mode(&stdin, &mut buf_vec);
        insert_at_index(0, buf_vec, contents);
    }
    Ok(None)
}

fn append(
    contents: &mut Vec<String>, 
    stdin: &Stdin
) -> Result<Option<ReturnType>, ErrorType> {
    let mut buf_vec: Vec<String> = Vec::new();
    input_mode(&stdin, &mut buf_vec);
    append_to_end(buf_vec, contents);
    Ok(None)
}

fn print_lines(
    args_iter: &mut dyn Iterator<Item = &str>, 
    contents: &Vec<String>, 
) -> Result<Option<ReturnType>, ErrorType> {
    let mut to_return = ReturnType::new();

    if let Some(first_index) = args_iter.next() {
        if let Some(second_index) = args_iter.next() {
            let n1 = match first_index.parse::<usize>() {
                Ok(x) => x - 1,
                Err(_) => {
                    return Err(ErrorType::TypeErr);
                }
            };
            let n2 = match second_index.parse::<usize>() {
                Ok(x) => x - 1,
                Err(_) => {
                    return Err(ErrorType::TypeErr);
                }
            };

            if n2+1 > contents.len() {
                for line in &contents[n1..]{
                    to_return.msg.push_str(line);
                    to_return.msg.push('\n');
                }
            } else {
                for (index, line) in contents[n1..=n2].iter().enumerate() {
                    let index_with_offset = index + n1;

                    if index_with_offset == n2 {
                        to_return.msg.push_str(line);
                    } else {
                        to_return.msg.push_str(line);
                        to_return.msg.push('\n');
                    }
                }
            }
            Ok(Some(to_return))
        } else {
            let n = match first_index.parse::<usize>() {
                Ok(x) => x - 1,
                Err(_) => {
                    return Err(ErrorType::TypeErr);
                }
            };

            if contents.len() == 0 {
                return Err(ErrorType::FileEmpty);
            }

            if n >= contents.len() {
                return Err(ErrorType::RangeError);
            }

            let line = &contents[n];
            to_return.msg.push_str(line);
            Ok(Some(to_return))
        }
    } else {
        let line = &contents[0];
        to_return.msg.push_str(line);
        Ok(Some(to_return))
    }
}

fn find(
    args_iter: &mut dyn Iterator<Item = &str>, 
    contents: &Vec<String>, 
) -> Result<Option<ReturnType>, ErrorType> {
    if let Some(to_search) = args_iter.next() {
        let to_return = contents
            .iter()
            .filter(|line| line.contains(&to_search))
            .fold(String::new(), |mut acc, line| {
                if !acc.is_empty() {
                    acc.push('\n'); 
                }
                acc + line
        });
        return Ok(Some(ReturnType { msg: to_return }));
    }
    return Err(ErrorType::ArgCountErr);
}

fn move_lines(
    args_iter: &mut dyn Iterator<Item = &str>, 
    contents: &mut Vec<String>, 
) -> Result<Option<ReturnType>, ErrorType> {
    match (args_iter.next(), args_iter.next()) {
        (Some(first_index), Some(second_index)) => {
            if let Some(third_index) = args_iter.next() {
                let n1 = match first_index.parse::<usize>() {
                    Ok(x) => x - 1,
                    Err(_) => {
                        return Err(ErrorType::TypeErr);
                    }
                };
                let n2 = match second_index.parse::<usize>() {
                    Ok(x) => x - 1,
                    Err(_) => {
                        return Err(ErrorType::TypeErr);
                    }
                };
                let n3 = match third_index.parse::<usize>() {
                    Ok(x) => x - 1,
                    Err(_) => {
                        return Err(ErrorType::TypeErr);
                    }
                };


                let mut to_move = Vec::new();
                for _ in n1..=n2 {
                    to_move.push(contents.remove(n1));
                }

                for (offset, line) in to_move.into_iter().enumerate() {
                    contents.insert(n3 + offset , line);
                }

                Ok(None)
            } else {
                let n1 = match first_index.parse::<usize>() {
                    Ok(x) => x - 1,
                    Err(_) => {
                        return Err(ErrorType::TypeErr);
                    }
                };
                let n2 = match second_index.parse::<usize>() {
                    Ok(x) => x - 1,
                    Err(_) => {
                        return Err(ErrorType::TypeErr);
                    }
                };

                let line = contents.remove(n1);
                contents.insert(n2, line);
                Ok(None)
            }
        },
        _ => Err(ErrorType::ArgCountErr)
    }
}

fn delete(
    args_iter: &mut dyn Iterator<Item = &str>, 
    contents: &mut Vec<String>, 
) -> Result<Option<ReturnType>, ErrorType> {
    if let Some(first_index) = args_iter.next() {
        if let Some(second_index) = args_iter.next() {
            let n1 = match first_index.parse::<usize>() {
                Ok(x) => x - 1,
                Err(_) => {
                    return Err(ErrorType::TypeErr);
                }
            };
            let n2 = match second_index.parse::<usize>() {
                Ok(x) => x - 1,
                Err(_) => {
                    return Err(ErrorType::TypeErr);
                }
            };

            if n1 < n2 && n2 < contents.len() {
                contents.drain(n1..n2);
            } else {
                return Err(ErrorType::RangeError);
            }
            Ok(None)
        } else {
            let n = match first_index.parse::<usize>() {
                Ok(x) => x - 1,
                Err(_) => {
                    return Err(ErrorType::TypeErr);
                }
            };

            if n < contents.len() {
                contents.remove(n);
            } else {
                return Err(ErrorType::RangeError);
            }
            Ok(None)
        }
    } else {
        if contents.len() != 0 {
            contents.remove(0);
        } else {
            return Err(ErrorType::FileEmpty);
        }
        Ok(None)
    }
}

fn save(
    file_path: &str,
    contents: &mut Vec<String>
) -> Result<Option<ReturnType>, ErrorType> {
    if let Err(e) = write(&file_path, &contents) {
        return Err(ErrorType::WriteErr(Box::new(e)));
    };
    Ok(None)
}

fn command_handler(
    input: String, 
    stdin: &Stdin, 
    file_path: &str, 
    contents: &mut Vec<String>
) -> Result<Option<ReturnType>, ErrorType> {
    let args = input
            .split_whitespace()
            .collect::<Vec<&str>>();

    let mut args_iter = args.into_iter();

    match args_iter.next() {
        Some("i") => insert(&mut args_iter, contents, stdin),
        Some("a") => append(contents, stdin),
        Some("p") => print_lines(&mut args_iter, contents),
        Some("m") => move_lines(&mut args_iter, contents),
        Some("f") => find(&mut args_iter, contents),
        Some("d") => delete(&mut args_iter, contents),
        Some("s") => save(file_path, contents),
        Some("q") => return Ok(Some(ReturnType{msg: "kill".to_string()})),
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
            Ok(Some(x)) => {
                if x.msg == "kill".to_string() {
                    process::exit(0);
                }

                println!("{}", x.msg);
            },
            Err(ErrorType::WriteErr(e)) => {
                println!("{e:?}");
                process::exit(1);
            },
            Err(e) => println!("{e:?}"),
        }
    }
}