use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let Some(file_path) = args.next() else {
        return Err("no file path was found")?;
    };

    let program = fs::read(file_path)?;
    let mut tape = [0u8; 4096];
    bf::run(program.as_slice(), tape.as_mut_slice())?;
    Ok(())
}
