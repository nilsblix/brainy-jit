use std::error::Error;
use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    mode: Mode,
    #[arg(short, long, global = true, default_value_t = 1024 * 16)]
    tape_size: usize,
}

#[derive(Debug, Subcommand)]
enum Mode {
    Run {
        input: PathBuf,
        #[arg(short, long, value_enum, default_value = "jit")]
        backend: Backend,
    },
    Compile {
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
}

impl Mode {
    fn read_program(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let input = match self {
            Self::Run { input, backend: _ } => input,
            Self::Compile { input, output: _ } => input,
        };
        Ok(fs::read(input)?)
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Backend {
    Jit,
    Interpreter,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let program = args.mode.read_program()?;
    let bytes = program.as_slice();
    let mut tape = vec![0u8; args.tape_size];

    match args.mode {
        Mode::Run { input: _, backend: engine } => match engine {
            Backend::Jit =>
                Ok(bf::run_aarch64_jit(bytes, tape.as_mut_slice())?),
            Backend::Interpreter =>
                Ok(bf::run_interpreter(bytes, tape.as_mut_slice())?),
        },
        Mode::Compile { input: _, output } => {
            _ = output;
            todo!("compile output");
        },
    }
}
