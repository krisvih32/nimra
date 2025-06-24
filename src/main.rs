use std::{fs, io::{self, Write}};
mod lexer;
fn main() {
    let mut file = String::new();
    print!("Enter file name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut file).expect("Error: unable to read user input");
    let file = file.trim();

    let code = fs::read_to_string(file).expect("Error: unable to read file");
    let lexed = lexer::lex(code.as_str());
    println!("{:#?}", lexed);

}
