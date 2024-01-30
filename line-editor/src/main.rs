use std::process;

fn main() {
    loop {
        if let Err(e) = line_editor::process_input() {
            eprintln!("{e}");
            process::exit(1);
        };
    }
}
