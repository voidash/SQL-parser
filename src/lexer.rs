use crate::symbol;
use std::fmt;

use log::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scanner {
    message: String,
    tokens: Vec<symbol::Symbol>,
    pos: Pos,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Pos {
    cursor_l: usize,
    cursor_r: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LexerError {
    NotAllowedChar,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LexerError::NotAllowedChar => write!(f, "please use ascii character."),
        }
    }
}

impl Scanner {
    pub fn new(message: &str) -> Scanner {
        Scanner {
            message: message.to_lowercase().trim().to_string(),
            tokens: vec![],
            pos: Pos {
                cursor_l: 0,
                cursor_r: 0,
            },
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<symbol::Symbol>, LexerError> {
        debug!("Starting scanning message:\n`{}`", self.message);
        let mut chars = self.message.chars();

        loop {
            match chars.next() {
                Some(x) => {
                     if is_identifier_char(x) || is_operator(x) {
                        self.pos.cursor_r += 1;
                    } else {
                        match x {
                            ' ' | '\t' | '\r' | '\n' | '(' | ')' | ',' | ';' => {
                                if self.pos.cursor_l != self.pos.cursor_r {
                                    let word = self.message.get(self.pos.cursor_l..self.pos.cursor_r).unwrap();
                                    debug!("encounter `{}`, last word is `{}`", x, word);

                                    let mut is_multi_keyword = false;

                                    // if this char is delimiter, it must not be a multikeyword
                                    if !is_delimiter(x) {
                                        // if this is possible a multikeyword, search the following chars
                                        match symbol::check_multi_keywords_front(word) {
                                            // parts<Vec[u32]> for how many parts in this possible keyword
                                            Some(parts) => {
                                                debug!("The word `{}` might be a multikeyword", word);

                                                for keyword_total_parts in parts {
                                                    debug!("Assume this keyword has {} parts", keyword_total_parts);

                                                    // copy remaining chars for testing
                                                    let mut test_chars = chars.as_str().chars();
                                                    // for testing if the string a multikeyword. Insert the first word
                                                    // and a space already. (because start scanning from next word)
                                                    let mut test_str = String::from(format!("{} ", word));

                                                    // for checking a new word
                                                    let mut is_last_letter = false;

                                                    // record the right cursor position when checking if multikeyword
                                                    // if match a multikeyword, shift right cursor with steps
                                                    let mut step_counter = 0;

                                                    // How many words added in the test_str
                                                    // if the keyword is 3 parts, the following_parts should be 2
                                                    let mut following_parts = 0;

                                                    loop {
                                                        match test_chars.next() {
                                                            Some(y) => {
                                                                // A multikeyword should be all ASCII alphabetic character
                                                                if y.is_ascii_alphabetic() {
                                                                    if !is_last_letter {
                                                                        is_last_letter = true;
                                                                    }
                                                                    test_str.push(y);
                                                                } else {
                                                                    match y {
                                                                        ' ' | '\t' | '\r' | '\n' => {
                                                                            if is_last_letter {
                                                                                // from letter to space, count one
                                                                                following_parts += 1;
                                                                                // find enough parts, break earlier
                                                                                if following_parts
                                                                                    == keyword_total_parts - 1
                                                                                {
                                                                                    break; // loop
                                                                                }
                                                                                // add ` ` between words
                                                                                test_str.push(' ');
                                                                                is_last_letter = false
                                                                            }
                                                                        }
                                                                        // &, %, *, @, etc.
                                                                        // keywords must be letters
                                                                        _ => break, // loop
                                                                    }
                                                                }
                                                            }
                                                            None => break, // loop
                                                        }
                                                        step_counter += 1;
                                                    }

                                                    debug!("Checking `{}` ...", test_str);
                                                    match symbol::SYMBOLS.get(test_str.as_str()) {
                                                        // a multikeyword
                                                        Some(token) => {
                                                            debug!("Found keyword `{}`", test_str);
                                                            self.tokens.push(token.clone());

                                                            // shift the right cursor to the right of multikeyword
                                                            self.pos.cursor_r += step_counter;
                                                            // skip the chars included in this multikeyword
                                                            for _ in 0..step_counter {
                                                                chars.next();
                                                            }

                                                            is_multi_keyword = true;
                                                            break; // parts
                                                        }
                                                        None => debug!("`{}` not a keyword", test_str),
                                                    }
                                                }
                                            }
                                            None => {}
                                        }
                                    }

                                    // a single word
                                    if !is_multi_keyword {
                                        match symbol::SYMBOLS.get(word) {
                                            // either keyword
                                            Some(token) => {
                                                self.tokens.push(token.clone());
                                            }
                                            // or identifier
                                            None => {
                                                self.tokens.push(symbol::symbol(
                                                    word,
                                                    symbol::Token::Identifier,
                                                    symbol::Group::Identifier,
                                                ));
                                            }
                                        }
                                    }
                                }
                                if is_delimiter(x) {
                                    debug!("take `{}`", x);
                                    self.tokens.push(symbol::Symbol::match_delimiter(x).unwrap());
                                }
                                // set the cursor next to `x` in the right
                                self.pos.cursor_r += 1;
                                self.pos.cursor_l = self.pos.cursor_r;
                            }
                            // A special case
                            '*' => {
                                self.tokens.push(symbol::symbol(
                                    "*",
                                    symbol::Token::Identifier,
                                    symbol::Group::Identifier,
                                ));
                                self.pos.cursor_r += 1;
                                self.pos.cursor_l = self.pos.cursor_r;
                            }
                            _ => {
                                return Err(LexerError::NotAllowedChar);
                            }
                        }
                    }
                }
                // iter to the end
                None => {
                    break;
                }
            };
        }
        Ok(self.tokens.clone())
    }
}

fn is_identifier_char(ch: char) -> bool {
    ch.is_digit(10) || ch.is_ascii_alphabetic() || ch == '\'' || ch == '.' || ch == '"' || ch == '_'
}

fn is_operator(ch: char) -> bool {
    ch == '>' || ch == '=' || ch == '<' || ch == '-' || ch == '+'
}

fn is_delimiter(ch: char) -> bool {
    ch == '(' || ch == ')' || ch == ',' || ch == ';'
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;

    #[test]
    pub fn test_scan_tokens() {
        let message = "select beautiful_girls from Computer_engineering where sem = 1;";
        let mut s = Scanner::new(message);
        let tokens = s.scan_tokens().unwrap();

        println!("{:?}", tokens);

    }

    #[test]
    fn test_scan_tokens_error() {
        let message = "create table $1234";
        let mut s = Scanner::new(message);
        match s.scan_tokens() {
            Ok(_) => {}
            Err(e) => assert_eq!(format!("{}", e), "please use ascii character."),
        }
    }
}
