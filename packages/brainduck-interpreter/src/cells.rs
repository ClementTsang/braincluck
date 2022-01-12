use std::collections::VecDeque;

use crate::Command;

/// [`Cells`] are an array of memory cells that Brainfuck commands can be applied to.
/// This array can continuously grow.
///
/// To initialize, either initialize with `Cells::with_capacity` with your
/// desired capacity, or `Cells::default()`, which will initialize the memory with the capacity
/// for 30 000 cells.
///
/// Implementation-wise, this is just a wrapper around a deque with a tracked index.
#[derive(Debug, Clone)]
pub struct Cells {
    memory: VecDeque<i8>,
    index: usize,
}

impl Cells {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            memory: {
                let mut mem = VecDeque::with_capacity(capacity);
                mem.push_back(0);
                mem
            },
            index: 0,
        }
    }

    pub fn left(&mut self) {
        if self.index == 0 {
            self.memory.push_front(0);
        } else {
            self.index -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.index < isize::MAX as usize {
            self.index += 1;
            if self.index == self.memory.len() {
                self.memory.push_back(0);
            }
        }
    }

    pub fn increment(&mut self) {
        self.memory[self.index] = self.memory[self.index].wrapping_add(1);
    }

    pub fn decrement(&mut self) {
        self.memory[self.index] = self.memory[self.index].wrapping_sub(1);
    }

    pub fn output(&self) -> Option<char> {
        u8::try_from(self.memory[self.index])
            .ok()
            .map(|num| char::from(num))
    }

    pub fn input(&mut self, input: i8) {
        self.memory[self.index] = input;
    }

    pub fn current(&self) -> i8 {
        self.memory[self.index]
    }

    pub fn is_current_cell_zero(&self) -> bool {
        self.memory[self.index] == 0
    }

    pub fn debug(&self) {}

    /// Given a list of commands, applies commands.
    pub fn interpret(&mut self, commands: &[Command]) {
        for command in commands {
            match command {
                Command::Right => self.right(),
                Command::Left => self.left(),
                Command::Increment => self.increment(),
                Command::Decrement => self.decrement(),
                Command::Output => {
                    if let Some(c) = self.output() {
                        // TODO: Output
                    }
                }
                Command::Input => {
                    // TODO: Input
                }
                Command::Jump(block) => {
                    while !self.is_current_cell_zero() {
                        self.interpret(block);
                    }
                }
            }
        }
    }
}

impl Default for Cells {
    fn default() -> Self {
        Self::with_capacity(30000)
    }
}

#[cfg(test)]
mod tests {
    use crate::{bf_parse, Cells};

    /// Straightforward hello world.
    #[test]
    fn hello_world() {
        let mut cells = Cells::default();
        let program = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

        let commands = bf_parse(program).expect("hello world parsing returned an error");

        cells.interpret(&commands);
    }

    /// This test can fail if cell values cannot be set below zero.
    #[test]
    fn tricky_hello_world() {
        let mut cells = Cells::default();
        let program = ">++++++++[-<+++++++++>]<.>>+>-[+]++>++>+++[>[->+++<<+++>]<<]>-----.>->
        +++..+++.>-.<<+[>[+>+]>>]<--------------.>>.+++.------.--------.>+.>+.";

        let commands = bf_parse(program).expect("tricky hello world parsing returned an error");

        cells.interpret(&commands);
    }

    /// A program to write hello world but requires wrapping cells.
    #[test]
    fn wrapping_hello_world() {
        let mut cells = Cells::default();
        let program =
            "--<-<<+[+[<+>--->->->-<<<]>]<<--.<++++++.<<-..<<.<+.>>.>>.<<<.+++.>>.>>-.<<<+.";
        let commands = bf_parse(program).expect("wrapping hello world parsing returned an error");

        cells.interpret(&commands);
    }

    /// A program to write hello world using the shortest code golf example. Requires wrapping cells.
    #[test]
    fn short_hello_world() {
        let mut cells = Cells::default();
        let program = "+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.";
        let commands = bf_parse(program).expect("short hello world parsing returned an error");

        cells.interpret(&commands);
    }

    #[test]
    fn cell_size() {
        let mut cells = Cells::default();
        let program = r##"
        Calculate the value 256 and test if it's zero
        If the interpreter errors on overflow this is where it'll happen
        ++++++++[>++++++++<-]>[<++++>-]
        +<[>-<
            Not zero so multiply by 256 again to get 65536
            [>++++<-]>[<++++++++>-]<[>++++++++<-]
            +>[>
                # Print "32"
                ++++++++++[>+++++<-]>+.-.[-]<
            <[-]<->] <[>>
                # Print "16"
                +++++++[>+++++++<-]>.+++++.[-]<
        <<-]] >[>
            # Print "8"
            ++++++++[>+++++++<-]>.[-]<
        <-]<
        # Print " bit cells\n"
        +++++++++++[>+++>+++++++++>+++++++++>+<<<<-]>-.>-.+++++++.+++++++++++.<.
        >>.++.+++++++..<-.>>-
        Clean up used cells.
        [[-]<]
        "##;

        let commands = bf_parse(program).expect("cell size parsing returned an error");
        cells.interpret(&commands);
    }
}
