use crate::math_expression_tokenizer::{
    MathExpressionTokenizer, MathExpressionTokenizerErrors, Token,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MathExpressionParserErrors {
    #[error("Invalid argument")]
    InvalidArgument,
    #[error("Found invalid token '{ch}' at position {idx}")]
    InvalidToken { idx: usize, ch: char },
    #[error("Invalid Expression by index '{idx}'")]
    InvalidExpression { idx: usize },
}

impl From<MathExpressionTokenizerErrors> for MathExpressionParserErrors {
    fn from(err: MathExpressionTokenizerErrors) -> Self {
        match err {
            MathExpressionTokenizerErrors::InvalidArgument => {
                MathExpressionParserErrors::InvalidArgument
            }
            MathExpressionTokenizerErrors::InvalidToken { idx, ch } => {
                MathExpressionParserErrors::InvalidToken { idx, ch }
            }
            _ => unreachable!("Unexpected tokenizer error"),
        }
    }
}

pub struct MathExpression {
    pub expression: Vec<Token>,
}

pub struct MathExpressionParser;

impl MathExpressionParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, expr: String) -> Result<MathExpression, MathExpressionParserErrors> {
        let expr_len = expr.as_bytes().len();
        let mut tokenizer = MathExpressionTokenizer::new(expr)?;
        let mut parsed_expression = MathExpression { expression: vec![] };
        let mut braces = vec![];

        while let Ok((token, idx)) = tokenizer.next_token() {
            match token {
                Token::OpenBrace => {
                    braces.push(idx);
                    if let Some(last_token) = parsed_expression.expression.last() {
                        if !matches!(last_token, Token::Operator(_) | Token::OpenBrace) {
                            return Err(MathExpressionParserErrors::InvalidExpression { idx });
                        };
                    }
                }
                Token::CloseBrace => {
                    if braces.pop().is_none() {
                        return Err(MathExpressionParserErrors::InvalidExpression { idx });
                    }

                    let Some(last_token) = parsed_expression.expression.last() else {
                        return Err(MathExpressionParserErrors::InvalidExpression { idx });
                    };

                    if !matches!(last_token, Token::Digit(_) | Token::CloseBrace) {
                        return Err(MathExpressionParserErrors::InvalidExpression { idx });
                    }
                }
                Token::Digit(_) => {
                    if let Some(last_token) = parsed_expression.expression.last() {
                        if !matches!(last_token, Token::Operator(_) | Token::OpenBrace) {
                            return Err(MathExpressionParserErrors::InvalidExpression { idx });
                        };
                    }
                }
                Token::Operator(op) => {
                    let Some(last_token) = parsed_expression.expression.last() else {
                        return Err(MathExpressionParserErrors::InvalidExpression { idx });
                    };

                    if !matches!(last_token, Token::Digit(_) | Token::CloseBrace) {
                        return Err(MathExpressionParserErrors::InvalidExpression { idx });
                    }
                }
            }

            parsed_expression.expression.push(token);
        }

        if let Some(last_token) = parsed_expression.expression.last() {
            if matches!(last_token, Token::Operator(_) | Token::OpenBrace) {
                return Err(MathExpressionParserErrors::InvalidExpression {
                    idx: expr_len - 1,
                });
            }
        }

        if braces.is_empty() {
            Ok(parsed_expression)
        } else {
            Err(MathExpressionParserErrors::InvalidExpression {
                idx: *braces.last().unwrap(),
            })
        }
    }
}
