mod file_handling;
mod lexer;
mod parser;
fn main() {
    let code = file_handling::get_code();
    println!("{}", code);
    let tokens = lexer::lex(code.as_str());
    println!("{:#?}", tokens);
    let ast = parser::parse(&tokens);
    println!("{:#?}", ast);
}
