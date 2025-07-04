mod codegen;
mod compile_c;
mod file_handling;
mod generator;
mod lexer;
mod parser;

fn main() {
    let code = match file_handling::get_code() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    let tokens = lexer::lex(code.as_str());
    let ast = parser::parse(&tokens);
    let ic = match generator::generate(ast) {
        Ok(ic) => ic,
        Err(e) => {
            eprintln!("IC generation error: {e}");
            return;
        }
    };
    let c = match codegen::codegen(ic) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Code generation error: {e}");
            return;
        }
    };
    let output_file = match compile_c::compile(c.as_str()) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Compilation error: {e}");
            return;
        }
    };
    let output_file_result_string = output_file.into_os_string().into_string();
    let output_file_real_string = match output_file_result_string {
        Ok(s) => s,
        Err(_) => {
            eprintln!("ERROR Output file result string");
            return;
        }
    };
    println!("{output_file_real_string}");
}
