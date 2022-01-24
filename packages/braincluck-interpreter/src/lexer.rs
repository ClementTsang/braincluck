pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Copy, Clone, Debug)]
pub enum Tok {
    GreaterThan,
    LessThan,
    Plus,
    Minus,
    Period,
    Comma,
    LeftBracket,
    RightBracket,
}

#[derive(Debug)]
pub enum LexicalError {
    // Can't occur
}

use std::str::CharIndices;

pub struct Lexer<'input> {
    chars: CharIndices<'input>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices(),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((i, '>')) => return Some(Ok((i, Tok::GreaterThan, i + 1))),
                Some((i, '<')) => return Some(Ok((i, Tok::LessThan, i + 1))),
                Some((i, '+')) => return Some(Ok((i, Tok::Plus, i + 1))),
                Some((i, '-')) => return Some(Ok((i, Tok::Minus, i + 1))),
                Some((i, '.')) => return Some(Ok((i, Tok::Period, i + 1))),
                Some((i, ',')) => return Some(Ok((i, Tok::Comma, i + 1))),
                Some((i, '[')) => return Some(Ok((i, Tok::LeftBracket, i + 1))),
                Some((i, ']')) => return Some(Ok((i, Tok::RightBracket, i + 1))),

                None => return None, // End of file
                _ => continue,       // Comment; skip this character
            }
        }
    }
}

#[test]
fn skip_comments() {
    let input = r##"
    Code:   Pseudo code:
    >>      Move the pointer to cell2
    [-]     Set cell2 to 0 
    <<      Move the pointer back to cell0
    [       While cell0 is not 0
    -       Subtract 1 from cell0
    >>      Move the pointer to cell2
    +       Add 1 to cell2
    <<      Move the pointer back to cell0
    ]       End while
    "##;

    assert_eq!(Lexer::new(input).count(), 15);
}
