use thiserror::Error;

#[derive(Debug)]
pub enum Token {
    Digit(f64),
    Operator(char),
    OpenBrace,
    CloseBrace,
}

#[derive(Debug, Error)]
pub enum MathExpressionTokenizerErrors {
    #[error("Invalid argument")]
    InvalidArgument,
    #[error("Found invalid token '{ch}' at position {idx}")]
    InvalidToken { idx: usize, ch: char },
    #[error("Token not found")]
    NoToken,
}

pub struct MathExpressionTokenizer {
    m_math_expr: String,
    curr_idx: usize,
}

impl MathExpressionTokenizer {
    pub fn new(math_expr: String) -> Result<Self, MathExpressionTokenizerErrors> {
        if math_expr.is_empty() {
            return Err(MathExpressionTokenizerErrors::InvalidArgument);
        }

        Ok(Self {
            m_math_expr: math_expr,
            curr_idx: 0,
        })
    }

    pub fn has_token(&self) -> bool {
        let idx = self.skip_spaces();
        return idx < self.m_math_expr.as_bytes().len();
    }

    pub fn next_token(&mut self) -> Result<Token, MathExpressionTokenizerErrors> {
        if !self.has_token() {
            return Err(MathExpressionTokenizerErrors::NoToken);
        }

        self.curr_idx = self.skip_spaces();

        match self.m_math_expr.as_bytes()[self.curr_idx] {
            b'(' => {
                self.curr_idx += 1;
                Ok(Token::OpenBrace)
            }
            b')' => {
                self.curr_idx += 1;
                Ok(Token::CloseBrace)
            }
            op @ (b'+' | b'-' | b'*' | b'/') => {
                self.curr_idx += 1;
                Ok(Token::Operator(op as char))
            }
            _ => {
                let digit = self.parse_digits()?;
                Ok(Token::Digit(digit))
            }
        }
    }

    fn parse_digits(&mut self) -> Result<f64, MathExpressionTokenizerErrors> {
        let mut tmp = String::new();
        let bytes = self.m_math_expr.as_bytes();

        while self.curr_idx < bytes.len()
            && (bytes[self.curr_idx].is_ascii_digit() || bytes[self.curr_idx] == b'.')
        {
            tmp.push(bytes[self.curr_idx] as char);

            self.curr_idx += 1;
        }

        match tmp.parse::<f64>() {
            Ok(number) => Ok(number),
            Err(_) => Err(MathExpressionTokenizerErrors::InvalidToken {
                idx: self.curr_idx - tmp.len(),
                ch: bytes[self.curr_idx - tmp.len()] as char,
            }),
        }
    }

    fn skip_spaces(&self) -> usize {
        if let Some(idx) = self.m_math_expr.as_bytes()[self.curr_idx..]
            .iter()
            .position(|x| !x.is_ascii_whitespace())
        {
            return if self.curr_idx + idx < self.m_math_expr.bytes().len() {
                self.curr_idx + idx
            } else {
                self.m_math_expr.as_bytes().len()
            };
        };

        self.m_math_expr.as_bytes().len()
    }
}
