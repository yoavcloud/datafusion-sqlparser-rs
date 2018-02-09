use std::cmp::PartialEq;
use std::fmt::Debug;
use std::iter::Peekable;
use std::str::Chars;

use super::tokenizer::*;

pub struct GenericTokenizer {}

impl<S,TE> SQLTokenizer<S,TE> for GenericTokenizer
    where S: Debug + PartialEq {

    fn next_token(&self, chars: &mut Peekable<Chars>) -> Result<Option<SQLToken<S>>, TokenizerError<TE>> {
        match chars.next() {
            Some(ch) => match ch {
                ' ' | '\t' | '\n' => Ok(Some(SQLToken::Whitespace(ch))),
                '0' ... '9' => {
                    let mut s = String::new();
                    s.push(ch);
                    while let Some(&ch) = chars.peek() {
                        match ch {
                            '0' ... '9' => {
                                chars.next(); // consume
                                s.push(ch);
                            },
                            _ => break
                        }
                    }
                    Ok(Some(SQLToken::Literal(s)))
                },
                '+' => Ok(Some(SQLToken::Plus)),
                '-' => Ok(Some(SQLToken::Minus)),
                '*' => Ok(Some(SQLToken::Mult)),
                '/' => Ok(Some(SQLToken::Divide)),
                _ => Err(TokenizerError::UnexpectedChar(ch,Position::new(0, 0)))
            },
            None => Ok(None)
        }
    }
}
