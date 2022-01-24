use std::io::{stdin, stdout};

use braincluck_interpreter::{bf_parse, Cells};

fn main() {
    let mut cells = Cells::default();
    let program = ",[.,]";
    let commands = bf_parse(program).unwrap();
    let mut out = stdout();
    let mut input = stdin();
    cells.interpret(&commands, &mut out, &mut input).unwrap();
}
