const VALID_TOKENS: &[u8] = b"><+-.,[]";

struct Lexer<'p> {
    idx: usize,
    program: &'p [u8],
}

impl<'p> Lexer<'p> {
    fn new(program: &'p [u8]) -> Self {
        Self { idx: 0, program }
    }

    fn at_valid_token(&self) -> bool {
        VALID_TOKENS.contains(&self.program[self.idx])
    }

    fn peek(&self) -> Option<(u8, usize)> {
        let mut idx = self.idx;
        while idx < self.program.len() && !self.at_valid_token() {
            idx += 1;
        }

        if idx >= self.program.len() {
            return None;
        }

        let ret = self.program[idx];
        idx += 1;
        Some((ret, idx))
    }

    fn next(&mut self) -> Option<u8> {
        let (ret, new_idx) = self.peek()?;
        self.idx = new_idx;
        Some(ret)
    }

    fn next_straight(&mut self) -> Option<(u8, usize)> {
        let byte = self.next()?;
        let mut count = 1;

        while self.peek().is_some_and(|x| x.0 == byte) {
            _ = self.next();
            count += 1;
        }

        if self.peek().is_none() {
            return None;
        }

        Some((byte, count))
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Right(u8),
    Left(u8),
    Increment(u8),
    Decrement(u8),
    Output(u8),
    Input(u8),
    JumpForward(usize),
    JumpBackward(usize),
}

type Ops = Vec<Op>;

fn lower_to_ops(program: &[u8]) -> Result<Ops, &'static str> {
    let mut lexer = Lexer::new(program);

    let mut ops = Vec::new();
    let mut jump_stack = Vec::new();

    loop {
        let Some((byte, count)) = lexer.next_straight() else {
            break;
        };
        match byte {
            b'>' => ops.push(Op::Right(count as u8)),
            b'<' => ops.push(Op::Left(count as u8)),
            b'+' => ops.push(Op::Increment(count as u8)),
            b'-' => ops.push(Op::Decrement(count as u8)),
            b'.' => ops.push(Op::Output(count as u8)),
            b',' => ops.push(Op::Input(count as u8)),
            b'[' => {
                jump_stack.push(ops.len());
                ops.push(Op::JumpForward(0));
            }
            b']' => {
                let Some(addr) = jump_stack.pop() else {
                    return Err("unbalanced jumps: found more ']' than '['");
                };
                ops[addr] = Op::JumpForward(ops.len());
                ops.push(Op::JumpBackward(addr));
            }
            _ => unreachable!(
                "lexer should never return a byte deviating from VALID_TOKENS"),
        }
    }

    if !jump_stack.is_empty() {
        return Err("unbalanced jumps: found more '[' than ']'");
    }

    Ok(ops)
}

struct RuntimeState {
    program_counter: usize,
    ptr: usize,
}

impl RuntimeState {
    fn new() -> Self {
        Self { program_counter: 0, ptr: 0 }
    }

    fn execute_op_and_advance(
        &mut self,
        op: Op,
        tape: &mut [u8]
    ) -> Result<(), &'static str> {
        use std::io::{Write, Read};
        match op {
            Op::Right(n) => self.ptr += n as usize,
            Op::Left(n) => self.ptr = self.ptr
                .checked_sub(n as usize)
                .ok_or("runtime failure: program counter less than 0")?,
            Op::Increment(n) => tape[self.ptr] = tape[self.ptr].overflowing_add(n).0,
            Op::Decrement(n) => tape[self.ptr] = tape[self.ptr].overflowing_sub(n).0,
            Op::Output(n) => for _ in 0..n {
                let buf = [tape[self.ptr]];
                let mut stdout = std::io::stdout();
                _ = stdout.write(&buf);
                _ = stdout.flush();
            },
            Op::Input(n) => for _ in 0..n {
                let mut stdin = std::io::stdin().lock();
                let mut buf = [0u8; 1];
                loop {
                    let i = stdin.read(&mut buf).map_err(|_| "runtime failure: io error")?;
                    if i == 1 {
                        break;
                    }
                }
                tape[self.ptr] = buf[0];
            },
            Op::JumpForward(addr) => if tape[self.ptr] == 0 {
                self.program_counter = addr;
            },
            Op::JumpBackward(addr) => if tape[self.ptr] != 0 {
                self.program_counter = addr;
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

pub fn run(program: &[u8], tape: &mut [u8]) -> Result<(), &'static str> {
    let ops = lower_to_ops(program)?;
    let mut runtime = RuntimeState::new();
    runtime.execute_ops(ops.as_slice(), tape)
}
