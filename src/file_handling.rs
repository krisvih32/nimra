use std::{env, fs};

pub fn get_code() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => fs::read_to_string(args[1].clone())
            .map_err(|e| format!("Error: Unable to read file or directory: {e}")),
        _ => fs::read_to_string(String::from("tests/exit.nimra"))
            .map_err(|e| format!("Error: Unable to read file or directory: {e}")),
    }
}
