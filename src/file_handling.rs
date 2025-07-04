use std::{env, fs};
pub fn get_code() -> String {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => fs::read_to_string(args[1].clone()).expect("Error: Unable to read file or directory"),
        // TODO: If release, uncomment this
        // _ => panic!("Wrong number of arguments, only pass file"),
        _ => fs::read_to_string(String::from("tests/exit.nimra").clone())
            .expect("Error: Unable to read file or directory"),
    }
}
