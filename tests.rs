use gag;
use std::io;
use std::io::{Read, Write};

use super::run_aarch64_jit;
use super::run_interpreter;

fn capture_stdout(run: impl FnOnce()) -> io::Result<Vec<u8>> {
    io::stdout().flush()?;
    let capture = gag::BufferRedirect::stdout()?;

    run();

    io::stdout().flush()?;
    let mut buffer = capture.into_inner();

    let mut output = Vec::new();
    buffer.read_to_end(&mut output)?;
    Ok(output)
}

const HELLO_WORLD: &'static [u8] = b"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

#[test]
fn interpret_hello_world() {
    let output = capture_stdout(|| {
        let mut tape = [0u8; 4096];
        run_interpreter(HELLO_WORLD, tape.as_mut_slice()).unwrap();
    })
    .unwrap();

    assert_eq!(output, b"Hello World!\n");
}

#[test]
fn jit_hello_world() {
    let output = capture_stdout(|| {
        let mut tape = [0u8; 4096];
        run_aarch64_jit(HELLO_WORLD, tape.as_mut_slice()).unwrap();
    })
    .unwrap();

    assert_eq!(output, b"Hello World!\n");
}
