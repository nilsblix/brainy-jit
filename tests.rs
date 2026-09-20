use std::io;
use std::io::{Read, Write};
use gag;

use super::run_with_interpreter;

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

#[test]
fn hello_world() {
    let output = capture_stdout(|| {
        let program = b"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let mut tape = [0u8; 4096];
        run_with_interpreter(program.as_slice(), tape.as_mut_slice()).unwrap();
    }).unwrap();

    assert_eq!(output, b"Hello World!\n");
}
