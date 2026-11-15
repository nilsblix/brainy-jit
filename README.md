# Brainy JIT

> Brainfuck JIT Compiler/Interpreter

This project was written for fun, and does not have any practical applications.

## Usage

**Dependencies:** `Cargo`, `macOS` (for compilation).

`Brainy JIT` can either `compile` or `interpret` Brainfuck programs.
Compilation can only be done on `macOS` due to us outputting `Mach-O`
executables. `JIT interpretation` could in theory run on any `aarch64` system,
however this claim is untested.

Examples:

```console
$ cargo r -q -- run --backend=interpreter bf/hello.bf
Hello World!
$ cargo r -q -- run --backend=jit bf/hello.bf
Hello World!
$ cargo r -q -- compile bf/hello.bf --output exe
$ ./exe
Hello World!
```

## Performance

On my 2021 Macbook M1 Pro I get these results when running `bf/mandelbrot.bf`:

```console
$ time ./target/release/cli run --backend=interpreter bf/mandelbrot.bf > /dev/null

________________________________________________________
Executed in    6.52 secs    fish           external
   usr time    6.44 secs    0.30 millis    6.44 secs
   sys time    0.07 secs    2.14 millis    0.07 secs

$ time ./target/release/cli run --backend=jit bf/mandelbrot.bf > /dev/null

________________________________________________________
Executed in  772.60 millis    fish           external
   usr time  739.89 millis    0.31 millis  739.58 millis
   sys time   29.31 millis    2.06 millis   27.25 millis

$ ./target/release/cli compile bf/mandelbrot.bf --output exe
$ time ./exe > /dev/null

________________________________________________________
Executed in    1.12 secs      fish           external
   usr time  741.57 millis    0.30 millis  741.27 millis
   sys time   30.85 millis    2.06 millis   28.79 millis
```

Note that running the mandelbrot program through JIT is ~9 times faster than
regular interpretation, and that the difference between JIT compilation and
running a compiled program is negligible.



