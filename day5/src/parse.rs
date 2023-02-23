use std::{
    cell::Cell,
    fmt::{Debug, Display},
    str::{Chars, FromStr},
};
use thiserror::Error;

#[derive(Copy, Clone)]
pub struct Crate(pub u8);

#[derive(Debug, Clone)]
pub struct Move {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, Clone)]
pub struct Parse {
    pub yard_cols: Vec<Vec<Crate>>,
    pub moves: Vec<Move>,
}

#[derive(Debug, Error)]
pub enum ParseError<'a> {
    #[error("Unexpected EOF")]
    UnexpectedEOF,
    #[error("Character '{0}' was unexpected")]
    Unexpected(char),
    #[error("Expected '{wanted}', but found '{found}' instead")]
    ExpectedExact { wanted: char, found: char },
    #[error(r#"Expected "{wanted}", but found "{found}" instead"#)]
    ExpectedExactStr {
        wanted: &'static str,
        found: &'a str,
    },
    #[error("Expected an alphabetic character for the crate name, found '{0}' instead")]
    ExpectedCrate(char),
    #[error(r#"Expected a number, but found "{0}" instead"#)]
    ExpectedNumber(&'a str),
}

impl Crate {
    fn from_char(c: char) -> Option<Self> {
        c.try_into().ok().map(Crate)
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0 as char)
    }
}

impl Debug for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Crate({})", self.0 as char)
    }
}

pub fn parse<'a>(input: &'a str) -> Result<Parse, ParseError<'a>> {
    // Convert rows to cols

    let mut char_iter = input.chars();

    let parse_result;
    let yard_cols = {
        // Parse the yard rows, and turn them into
        // yard columns
        let mut yard_rows = Vec::with_capacity(8);
        parse_result = parse_rows(&mut char_iter, &mut yard_rows);

        let col_count = yard_rows.first().map_or(0, |x| x.len());
        (0..col_count)
            .map(|x| {
                yard_rows
                    .iter()
                    .rev()
                    .flat_map(|row| row.get(x).copied())
                    .flatten()
                    .collect()
            })
            .collect()
    };

    // If we have an unexpected character in the input
    // we try to move on to the next stage of the parser
    match parse_result {
        Err(ParseError::Unexpected(_)) => {}
        Err(x) => return Err(x),
        _ => {}
    };

    // Skip the current line (as it should just be the
    // numbering line)
    while let Some(x) = char_iter.next() {
        if x == '\n' {
            break;
        }
    }

    parse_moves(&char_iter).map(|moves| Parse { yard_cols, moves })
}

fn parse_rows<'a, 'b, 's>(
    iter: &'a mut Chars<'s>,
    yard_rows: &'b mut Vec<Vec<Option<Crate>>>,
) -> Result<(), ParseError<'s>> {
    use ParseError as PE;

    yard_rows.clear();
    yard_rows.reserve(8);

    let mut iter = iter.by_ref().peekable();

    let want_new_row = Cell::new(true);
    let mut push_item = |x| {
        if want_new_row.replace(false) {
            yard_rows.push(Vec::with_capacity(16))
        }

        yard_rows.last_mut().unwrap().push(x);
    };

    while let Some(curr) = iter.next() {
        match curr {
            ' ' => {
                // If we recieve 3 spaces in a row, we have an empty crate
                let Some(_) = iter.next_if_eq(&' ') else { continue };
                let Some(_) = iter.next_if_eq(&' ') else { continue };

                push_item(None);
                // Consume a space if there is one after the crate
                iter.next_if_eq(&' ');
            }

            '[' => {
                let crate_id = match iter.next() {
                    Some(x) if x.is_alphabetic() => x,
                    Some(x) => return Err(ParseError::ExpectedCrate(x)),
                    _ => return Err(ParseError::UnexpectedEOF),
                };

                match iter.next() {
                    Some(']') => {}
                    Some(x) => {
                        return Err(ParseError::ExpectedExact {
                            wanted: ']',
                            found: x,
                        })
                    }
                    None => return Err(ParseError::UnexpectedEOF),
                };

                push_item(Some(Crate::from_char(crate_id).unwrap()));

                // Consume a space if there is one after the crate
                iter.next_if_eq(&' ');
            }

            '\n' => {
                // We do not create and append a new vector here, instead,
                // we just set a flag saying we want a new vector *if any more
                // data appears*.
                //
                // We do not append the vector here because we could simply be reading blank
                want_new_row.set(true);
            }

            any => return Err(PE::Unexpected(any)),
        }
    }

    Ok(())
}

fn parse_moves<'a>(iter: &Chars<'a>) -> Result<Vec<Move>, ParseError<'a>> {
    let move_str = iter.as_str();
    let mut word_iter = move_str.split_ascii_whitespace();

    let mut moves = Vec::new();

    loop {
        match word_iter.next() {
            Some("move") => {}
            Some(_) => continue,
            None => break,
        }

        let count: usize = parse_number(word_iter.next())?;

        match word_iter.next() {
            Some("from") => {}
            Some(x) => {
                return Err(ParseError::ExpectedExactStr {
                    wanted: "from",
                    found: x,
                })
            }
            None => return Err(ParseError::UnexpectedEOF),
        }

        let from: usize = parse_number(word_iter.next())?;

        match word_iter.next() {
            Some("to") => {}
            Some(x) => {
                return Err(ParseError::ExpectedExactStr {
                    wanted: "to",
                    found: x,
                })
            }
            None => return Err(ParseError::UnexpectedEOF),
        }

        let to: usize = parse_number(word_iter.next())?;

        moves.push(Move { count, to, from });
    }

    Ok(moves)
}

fn parse_number<'a, I: FromStr>(string: Option<&'a str>) -> Result<I, ParseError<'a>> {
    match string {
        Some(word) => match word.parse() {
            Ok(x) => Ok(x),
            Err(_) => Err(ParseError::ExpectedNumber(word)),
        },
        None => Err(ParseError::UnexpectedEOF),
    }
}
