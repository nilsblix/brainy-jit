#[repr(u8)]
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    Right        = b'>',
    Left         = b'<',
    Increment    = b'+',
    Decrement    = b'-',
    Output       = b'.',
    Input        = b',',
    JumpForward  = b'[',
    JumpBackward = b']',
}

struct Lexer {
    idx: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(program: &[u8]) -> Self {
        let tokens: Vec<Token> = program
            .iter()
            .filter_map(|&b| Some(match b {
                b'>' => Token::Right,
                b'<' => Token::Left,
                b'+' => Token::Increment,
                b'-' => Token::Decrement,
                b'.' => Token::Output,
                b',' => Token::Input,
                b'[' => Token::JumpForward,
                b']' => Token::JumpBackward,
                _ => return None,
            }))
            .collect();
        Self { tokens, idx: 0 }
    }

    fn next(&mut self) -> Option<Token> {
        if self.idx >= self.tokens.len() {
            None
        } else {
            let ret = self.tokens[self.idx];
            self.idx += 1;
            Some(ret)
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Op {
    kind: Token,
    /// The payload means different things depending on the kind.
    ///
    /// For `><+-.,` it means the amount of times to repeat the operation, while
    /// for `[]` it means the address to jump to.
    ///
    /// Note that for `<>+-.,` the payload should always be castable to a u8,
    /// due to the tape works in Brainfuck.
    payload: usize,
}

type OpSequence = Vec<Op>;

fn lower_to_ops(program: &[u8]) -> Result<OpSequence, &'static str> {
    let mut lexer = Lexer::new(program);

    let mut ops = Vec::new();
    let mut jump_stack = Vec::new();

    let mut token = lexer.next();
    loop {
        if token.is_none() {
            break;
        }

        let unwrapped = token.unwrap();
        match unwrapped {
            Token::Right
            | Token::Left
            | Token::Increment
            | Token::Decrement
            | Token::Output
            | Token::Input => {
                let mut count = 1;
                let next_token = loop {
                    let t = lexer.next();
                    if t.is_none() || t.unwrap() != unwrapped {
                        break t;
                    }
                    count += 1;
                };

                ops.push(Op { kind: unwrapped, payload: count });
                token = next_token;
            },
            Token::JumpForward => {
                let addr = ops.len();
                jump_stack.push(addr);
                ops.push(Op { kind: unwrapped, payload: 0 });
                token = lexer.next();
            },
            Token::JumpBackward => {
                let Some(addr) = jump_stack.pop() else {
                    return Err("unbalanced loop: more ']' than '['");
                };
                ops.push(Op { kind: unwrapped, payload: addr + 1 });
                ops[addr].payload = ops.len();
                token = lexer.next();
            },
        }
    }

    if !jump_stack.is_empty() {
        return Err("unbalanced jumps: found more '[' than ']'");
    }

    Ok(ops)
}

struct RuntimeState {
    program_counter: usize,
    head: usize,
}

impl RuntimeState {
    fn new() -> Self {
        Self { program_counter: 0, head: 0 }
    }

    fn execute_op_and_advance(
        &mut self,
        op: Op,
        tape: &mut [u8],
    ) -> Result<(), &'static str> {
        use std::io::{Write, Read};

        let n = op.payload;
        match op.kind {
            Token::Right => self.head += n,
            Token::Left => self.head = self.head
                .checked_sub(n)
                .ok_or("runtime failure: program counter less than 0")?,
            Token::Increment =>
                tape[self.head] = tape[self.head].overflowing_add(n as u8).0,
            Token::Decrement =>
                tape[self.head] = tape[self.head].overflowing_sub(n as u8).0,
            Token::Output => {
                let buf = vec![tape[self.head]; n];
                std::io::stdout()
                    .write_all(&buf)
                    .map_err(|_| "runtime failure: io error")?;
                std::io::stdout()
                    .flush()
                    .map_err(|_| "runtime failure: io error")?;
            },
            Token::Input => {
                let mut stdin = std::io::stdin().lock();
                let mut buf = [0u8; 1];

                for _ in 0..n {
                    stdin
                        .read_exact(&mut buf)
                        .map_err(|_| "runtime failure: io error")?;
                    tape[self.head] = buf[0];
                }
            },
            Token::JumpForward => if tape[self.head] == 0 {
                self.program_counter = op.payload - 1;
            },
            Token::JumpBackward => if tape[self.head] != 0 {
                self.program_counter = op.payload - 1;
            },
        }

        self.program_counter += 1;
        Ok(())
    }

    fn execute_ops(
        &mut self,
        ops: &[Op],
        tape: &mut [u8]
    ) -> Result<(), &'static str> {
        while self.program_counter < ops.len() {
            let op = ops[self.program_counter];
            self.execute_op_and_advance(op, tape)?;
        }
        Ok(())
    }
}

pub fn run_with_interpreter(
    program: &[u8],
    tape: &mut [u8],
) -> Result<(), &'static str> {
    let ops = lower_to_ops(program)?;

    let mut runtime = RuntimeState::new();
    runtime.execute_ops(ops.as_slice(), tape)
}

#[cfg(test)]
mod tests;
