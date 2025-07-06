use thiserror::Error;

#[derive(Debug, PartialEq)]
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
    curr_byte_idx: usize,
}

pub trait TokenizerTraits {
    fn has_token(&self) -> bool;
    fn next_token(&mut self) -> Result<(usize, Token), MathExpressionTokenizerError>;
    fn curr_index(&self) -> usize;
}

impl TokenizerTraits for MathExpressionTokenizer {
    fn has_token(&self) -> bool {
        let idx = self.skip_spaces();
        return idx < self.expr.len();
    }

    fn curr_index(&self) -> usize {
        self.curr_byte_idx
    }

    fn next_token(&mut self) -> Result<(usize, Token), MathExpressionTokenizerError> {
        if !self.has_token() {
            return Err(MathExpressionTokenizerError::NoToken);
        }

        self.curr_byte_idx = self.skip_spaces();
        let old_value = self.curr_byte_idx;

        match self.expr[self.curr_byte_idx..].chars().next().unwrap() {
            '(' => Ok((
                std::mem::replace(&mut self.curr_byte_idx, old_value + 1),
                Token::OpenBrace,
            )),
            ')' => Ok((
                std::mem::replace(&mut self.curr_byte_idx, old_value + 1),
                Token::CloseBrace,
            )),
            op @ ('+' | '-' | '*' | '/') => Ok((
                std::mem::replace(&mut self.curr_byte_idx, old_value + 1),
                Token::Operator(op),
            )),
            _ => {
                let (digit, idx) = self.parse_digits()?;
                Ok((
                    std::mem::replace(&mut self.curr_byte_idx, idx),
                    Token::Digit(digit),
                ))
            }
        }
    }
}

impl MathExpressionTokenizer {
    pub fn new(expr: String) -> Result<Self, MathExpressionTokenizerError> {
        if expr.is_empty() {
            return Err(MathExpressionTokenizerError::InvalidArgument);
        }

        Ok(Self {
            expr,
            curr_byte_idx: 0,
        })
    }

    fn parse_digits(&self) -> Result<(f64, usize), MathExpressionTokenizerError> {
        let s = &self.expr[self.curr_byte_idx..];

        let offset = s
            .char_indices()
            .find(|&(_, ch)| !ch.is_digit(10) && ch != '.')
            .map(|(i, _)| i)
            .unwrap_or(s.len());

        match s[..offset].parse::<f64>() {
            Ok(number) => Ok((number, self.curr_byte_idx + offset)),
            Err(_) => Err(MathExpressionTokenizerError::InvalidToken {
                idx: self.curr_byte_idx,
                ch: s.chars().nth(0).unwrap(),
            }),
        }
    }

    fn skip_spaces(&self) -> usize {
        self.expr[self.curr_byte_idx..]
            .char_indices()
            .find(|(_, char)| !char.is_whitespace())
            .map(|(idx, _)| self.curr_byte_idx + idx)
            .unwrap_or(self.expr.len())
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_empty_string_tokens() {
        assert!(MathExpressionTokenizer::new("".to_string()).is_err());
    }

    #[test]
    fn test_zero_number_tokens() {
        let mut tokenizer = MathExpressionTokenizer::new("0".to_string()).unwrap();
        assert!(tokenizer.has_token());
        let (idx, token) = tokenizer.next_token().unwrap();
        assert_eq!(idx, 0);

        if let Token::Digit(number) = token {
            assert!((number - 0.0).abs() < f64::EPSILON);
        } else {
            panic!("Expected Token::Digit, got {:?}", token);
        }

        let mut tokenizer = MathExpressionTokenizer::new("-0".to_string()).unwrap();
        assert!(tokenizer.has_token());
        let (idx, token) = tokenizer.next_token().unwrap();
        assert_eq!(idx, 0);
        assert!(matches!(token, Token::Operator('-')));

        assert!(tokenizer.has_token());
        let (idx, token) = tokenizer.next_token().unwrap();
        assert_eq!(idx, 1);

        if let Token::Digit(number) = token {
            assert!((number - 0.0).abs() < f64::EPSILON);
        } else {
            panic!("Expected Token::Digit, got {:?}", token);
        }
    }

    proptest! {
        #[test]
        fn test_valid_positive_number_tokens(n in any::<f64>().prop_filter("Positive numbers", |&x| x > 0.0)) {
            let mut tokenizer = MathExpressionTokenizer::new(format!("{}", n)).unwrap();
            assert!(tokenizer.has_token());
            let (idx, token) = tokenizer.next_token().unwrap();
            assert_eq!(idx, 0);

            if let Token::Digit(number) = token
            {
                assert_eq!(number, n);
            }
            else {
                panic!("Expected Token::Digit, got {:?}", token);
            }
        }

        #[test]
        fn test_valid_negative_number_tokens(n in any::<f64>().prop_filter("Positive numbers", |&x| x < 0.0)) {
            let mut tokenizer = MathExpressionTokenizer::new(format!("{}", n)).unwrap();
            assert!(tokenizer.has_token());
            let (idx, token) = tokenizer.next_token().unwrap();
            assert_eq!(idx, 0);
            assert!(matches!(token, Token::Operator('-')));

            assert!(tokenizer.has_token());
            let (idx, token) = tokenizer.next_token().unwrap();
            assert_eq!(idx, 1);

            if let Token::Digit(number) = token
            {
                assert_eq!(number, n.abs());
            }
            else {
                panic!("Expected Token::Digit, got {:?}", token);
            }
        }

        #[test]
        fn test_valid_operator_tokens(s in r"[+\-*/\s]{1,50}".prop_filter("no leading space", |s| !s.starts_with(char::is_whitespace))) {
            let mut tokenizer = MathExpressionTokenizer::new(s.clone()).unwrap();
            assert!(tokenizer.has_token());

            while let Ok((idx, token)) = tokenizer.next_token() {
                let op = s[idx..].chars().next().unwrap();
                assert_eq!(token, Token::Operator(op));
            }

            assert!(!tokenizer.has_token());
        }

        #[test]
        fn test_braces_tokens(s in r"[()\s]{1,50}".prop_filter("no leading space", |s| !s.starts_with(char::is_whitespace))) {
            let mut tokenizer = MathExpressionTokenizer::new(s.clone()).unwrap();
            assert!(tokenizer.has_token());

            while let Ok((idx, token)) = tokenizer.next_token() {
                let op = s[idx..].chars().next().unwrap();
                if op == '('
                {
                    assert_eq!(token, Token::OpenBrace);
                }
                else if op == ')'
                {
                    assert_eq!(token, Token::CloseBrace);
                }
            }

            assert!(!tokenizer.has_token());
        }

        #[test]
        fn test_valid_sequence_tokens(s in r"[0-9+\-*/()\s]{1,10}".prop_filter("no leading space", |s| !s.starts_with(char::is_whitespace))) {
            let mut tokenizer = MathExpressionTokenizer::new(s.clone()).unwrap();
            assert!(tokenizer.has_token());

            while let Ok((idx, token)) = tokenizer.next_token() {
                    let ch = s[idx..].chars().next().unwrap();
                    match token {
                        Token::OpenBrace => {
                            assert_eq!(ch, '(');
                        },
                        Token::CloseBrace => {
                            assert_eq!(ch, ')');
                        },
                        Token::Operator(op) => {
                            assert_eq!(ch, op);
                        },
                        Token::Digit(_) => {
                            assert!(ch.is_digit(10));
                        },
                    }
                }

            assert!(!tokenizer.has_token());
        }
    }
}
