/*
 * Copyright (C) 2025 Vihaan Krishnan
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/agpl-3.0.html>.
 */

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
