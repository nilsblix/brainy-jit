use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);

    let Some(command) = args.next() else {
        return Err("no command was found")?;
    };

    let Some(file_path) = args.next() else {
        return Err("no file path was found")?;
    };

    let program = fs::read(file_path)?;
    let mut tape = [0u8; 4096];

    match command.as_str() {
        "int" => bf::run_interpreter(&program, &mut tape)?,
        "jit" => bf::run_aarch64_jit(&program, &mut tape)?,
        _ => return Err("unknown command")?,
    }

    Ok(())
}
