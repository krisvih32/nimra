mod file_handling;
mod lexer;
mod parser;
mod generator;
fn main() {
    let code = file_handling::get_code();
    let tokens = lexer::lex(code.as_str());
    let ast = parser::parse(&tokens);

    println!("{:#?}", ast);
    let ic = generator::generate(ast);
}
