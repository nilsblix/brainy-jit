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
            Self::Run { input, .. } | Self::Compile { input, .. } => input,
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

    match args.mode {
        Mode::Run { backend, .. } => {
            let mut tape = vec![0u8; args.tape_size];
            match backend {
                Backend::Jit => bf::run_aarch64_jit(bytes, &mut tape)?,
                Backend::Interpreter => bf::run_interpreter(bytes, &mut tape)?,
            }
        }
        Mode::Compile { output, .. } => {
            let blob = bf::compile_bf_into_aarch64_binary(bytes, args.tape_size)?;
            blob.dump_to_file(output)?;
        }
    }

    Ok(())
}
