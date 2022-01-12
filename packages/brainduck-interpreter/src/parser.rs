use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub bf);

#[cfg(test)]
mod tests {
    use crate::{bf_parse, Command};

    #[test]
    fn simple() {
        let input = "><+-.,[><]";
        assert_eq!(
            vec![
                Command::Right,
                Command::Left,
                Command::Increment,
                Command::Decrement,
                Command::Output,
                Command::Input,
                Command::Jump(vec![Command::Right, Command::Left,]),
            ],
            bf_parse(input).expect("simple parsing returned an error"),
            "simple parsing failed"
        );
    }

    #[test]
    fn hello_world() {
        let input = ">++++++++[-<+++++++++>]<.>>+>-[+]++>++>+++[>[->+++<<+++>]<<]>-----.>->
        +++..+++.>-.<<+[>[+>+]>>]<--------------.>>.+++.------.--------.>+.>+.";

        bf_parse(input).expect("hello world parsing returned an error");
    }

    #[test]
    fn newlines() {
        let input = ">++++++++[-<++++++\n+++>]<.>>+>-[+]++>++>+++[>[->+++<<+++>]<<]>-----.>->\n\n
        +++..+++.>-.<<+[>[+>+]>>]<--------------.>>.+++.------.--------.>+.>+.\n";

        bf_parse(input).expect("newline parsing returned an error");
    }
}
