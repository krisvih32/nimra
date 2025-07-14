use std::{env, fs};

pub fn get_code() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => fs::read_to_string(args[1].clone())
            .map_err(|e| format!("Error: Unable to read file or directory: {e}")),
        _ => Err("Error: Please provide exactly one argument (the file path)".to_string()),
    }
}
