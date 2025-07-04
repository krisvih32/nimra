use std::io::Write;
use std::process::{Command, Stdio};
extern crate tempfile;
use self::tempfile::Builder;
use std::path::PathBuf;

pub fn compile(c: &str) -> std::io::Result<PathBuf> {
    // 1. Create a temp source file with a .c extension
    let mut src = Builder::new().suffix(".c").tempfile()?;
    src.write_all(c.as_bytes())?;
    let src_path = src.path().to_path_buf();

    // 2. Create a temp output file for the binary
    let out_path = "./a.out";

    // 3. GCC flags
    let flags = [
        "-std=c2x", "-pedantic-errors", "-Wall", "-Wextra", "-Wconversion", "-Wshadow", "-Wstrict-aliasing=3",
        "-Wcast-align", "-Wcast-qual", "-Wwrite-strings", "-Wformat=2", "-Wswitch-enum", "-Wswitch-default",
        "-Wfloat-equal", "-Wundef", "-Wredundant-decls", "-Wpointer-arith", "-Winit-self", "-Wmissing-declarations",
        "-Wmissing-prototypes", "-Wstrict-prototypes", "-Wold-style-definition", "-Werror", "-fno-common", "-O3",
        "-flto", "-march=native", "-funroll-loops", "-fstack-protector-strong", "-fstack-clash-protection",
        "-D_FORTIFY_SOURCE=2", "-fPIC",
        "-fsanitize=undefined,address,leak,signed-integer-overflow,shift,alignment,bounds,object-size,float-divide-by-zero,float-cast-overflow",
        "-fno-omit-frame-pointer", "-fvisibility=hidden"
    ];

    // 4. Run gcc
    let status = Command::new("gcc")
        .args(flags)
        .arg(&src_path)
        .arg("-o")
        .arg(out_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        return Err(std::io::Error::other("gcc failed"));
    }

    Ok(PathBuf::from(out_path))
}
