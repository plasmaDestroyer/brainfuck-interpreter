# Brainfuck Interpreter

An Interpreter for Brainfuck built from scratch in Rust.

## Commands 
| Command | Description                                          |
| ------- | ---------------------------------------------------- |
| `>`     | Move pointer right                                   |
| `<`     | Move pointer left                                    |
| `+`     | Increment current cell                               |
| `-`     | Decrement current cell                               |
| `.`     | Output current cell as ASCII                         |
| `,`     | Read one byte of input                               |
| `[`     | Jump forward to matching `]` if current cell is 0    |
| `]`     | Jump back to matching `[` if current cell is nonzero |
## Usage

### Building

```
cargo build
```

### Running

```
cargo run -- your_brainfuck_file.bf
```

## Example

For Example, for this input file:
```my_input.bf
>+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.>>>++++++++[<++++>-]
<.>>>++++++++++[<+++++++++>-]<---.<<<<.+++.------.--------.>>+.>++++++++++.
```

And running the command:
```
cargo run -- my_input.bf
```

The generated output is:
```
Hello World!
```

## Specifications  

**Memory Model:** A linear tape of 30,000 cells, initialised to 0.

**Cell Size:** 8-bit unsigned integers (0–255), wrapping around.

**Pointer Behaviour:** Moving left from the first cell (index 0) and right from the last cell (index 29,999) is **NOT allowed**.

**Loops:** `[` and `]` allow nested loops, functioning similarly to `while(cell != 0)`.

**Complexity:** Turing-complete, meaning it can solve any computational problem given enough memory and time.