# brainduck-interpreter

A library for bf parser/interpreter, using [lalrpop](https://github.com/lalrpop/lalrpop). Somewhat overkill.

## Usage

You can feed in an input `&str` to get a `Vec<Command>`:

```rust
let commands : Vec<Command> = bf_parse(",[.,]")
```

You then need to initialize `Cells`, which represents your memory cell array:

```rust
let mut cells = Cells::default();
```

You also need something you can write to (implementing `std::io::Write`) and a reader (implementing `std::io::Read`):

```rust
let mut out = stdout();
let mut input = stdin();
```

You can then feed a reference to these commands, writer, and reader to these cells via `interpret`:

```rust
cells.interpret(&commands, &mut out, &mut input).unwrap();
```

A complete example demonstrating Hello World (source bf program from
[the Esolang wiki](https://esolangs.org/wiki/Brainfuck#Hello.2C_World.21)):

```rust
let mut cells = Cells::default();
let program = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
let commands = bf_parse(program).unwrap();
let mut out = stdout();
let mut input = stdin();
cells.interpret(&commands, &mut out, &mut input).unwrap();
```

See [here](../../examples/hello_world/) for an example.
