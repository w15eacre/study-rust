use thiserror::Error;

#[derive(Debug)]
pub enum Token {
    Digit(f64),
    Operator(char),
    OpenBrace,
    CloseBrace,
}

#[derive(Debug, Error)]
pub enum MathExpressionTokenizerError {
    #[error("Invalid argument")]
    InvalidArgument,
    #[error("Found invalid token '{ch}' at position {idx}")]
    InvalidToken { idx: usize, ch: char },
    #[error("Token not found")]
    NoToken,
}

pub struct MathExpressionTokenizer {
    expr: String,
    curr_idx: usize,
}

pub trait TokenizerTraits {
    fn has_token(&self) -> bool;
    fn next_token(&mut self) -> Result<(Token, usize), MathExpressionTokenizerError>;
    fn curr_index(&self) -> usize;
}

impl TokenizerTraits for MathExpressionTokenizer {
    fn has_token(&self) -> bool {
        let idx = self.skip_spaces();
        return idx < self.expr.as_bytes().len();
    }

    fn curr_index(&self) -> usize {
        self.curr_idx
    }

    fn next_token(&mut self) -> Result<(Token, usize), MathExpressionTokenizerError> {
        if !self.has_token() {
            return Err(MathExpressionTokenizerError::NoToken);
        }

        self.curr_idx = self.skip_spaces();

        match self.expr.as_bytes()[self.curr_idx] {
            b'(' => {
                self.curr_idx += 1;
                Ok((Token::OpenBrace, self.curr_idx - 1))
            }
            b')' => {
                self.curr_idx += 1;
                Ok((Token::CloseBrace, self.curr_idx - 1))
            }
            op @ (b'+' | b'-' | b'*' | b'/') => {
                self.curr_idx += 1;
                Ok((Token::Operator(op as char), self.curr_idx - 1))
            }
            _ => {
                let (digit, idx) = self.parse_digits()?;
                Ok((Token::Digit(digit), idx))
            }
        }
    }
}

impl MathExpressionTokenizer {
    pub fn new(math_expr: String) -> Result<Self, MathExpressionTokenizerError> {
        if math_expr.is_empty() {
            return Err(MathExpressionTokenizerError::InvalidArgument);
        }

        Ok(Self {
            expr: math_expr,
            curr_idx: 0,
        })
    }

    fn parse_digits(&mut self) -> Result<(f64, usize), MathExpressionTokenizerError> {
        let mut tmp = String::new();
        let bytes = self.expr.as_bytes();

        let begin = self.curr_idx;

        while self.curr_idx < bytes.len()
            && (bytes[self.curr_idx].is_ascii_digit() || bytes[self.curr_idx] == b'.')
        {
            tmp.push(bytes[self.curr_idx] as char);

            self.curr_idx += 1;
        }

        match tmp.parse::<f64>() {
            Ok(number) => Ok((number, begin)),
            Err(_) => Err(MathExpressionTokenizerError::InvalidToken {
                idx: begin,
                ch: bytes[begin] as char,
            }),
        }
    }

    fn skip_spaces(&self) -> usize {
        if let Some(idx) = self.expr.as_bytes()[self.curr_idx..]
            .iter()
            .position(|x| !x.is_ascii_whitespace())
        {
            return if self.curr_idx + idx < self.expr.bytes().len() {
                self.curr_idx + idx
            } else {
                self.expr.as_bytes().len()
            };
        };

        self.expr.as_bytes().len()
    }
}
