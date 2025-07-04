mod codegen;
mod compile_c;
mod file_handling;
mod generator;
mod lexer;
mod parser;

fn main() {
    let code = file_handling::get_code();
    let tokens = lexer::lex(code.as_str());
    let ast = parser::parse(&tokens);
    let ic = generator::generate(ast);
    let c = codegen::codegen(ic);
    let output_file = compile_c::compile(c.as_str());
    let mut output_file_string = output_file.expect("ERROR: File");
    let output_file_result_string = output_file_string.as_mut_os_string().clone().into_string();
    let output_file_real_string =
        output_file_result_string.expect("ERROR Output file result string");
    println!("{output_file_real_string}")
}
