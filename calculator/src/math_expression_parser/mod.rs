use crate::math_expression_tokenizer::{MathExpressionTokenizerError, Token, TokenizerTraits};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MathExpressionParserError {
    #[error("Tokenizer error: {0}")]
    Tokenizer(#[from] MathExpressionTokenizerError),
    #[error("Invalid expression by index '{idx}'")]
    InvalidExpression { idx: usize },
    #[error("Invalid braces consequence '{idx}'")]
    InvalidBraceConsequence { idx: usize },
}

pub struct MathExpression {
    pub expression: Vec<Token>,
}

pub struct MathExpressionParser;

impl MathExpressionParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse<Tokenizer: TokenizerTraits>(
        &self,
        mut tokenizer: Tokenizer,
    ) -> Result<MathExpression, MathExpressionParserError> {
        let mut parsed_expression = MathExpression { expression: vec![] };
        let mut braces = vec![];

        while let Ok((idx, token)) = tokenizer.next_token() {
            match token {
                Token::OpenBrace => {
                    braces.push(idx);
                    if let Some(last_token) = parsed_expression.expression.last() {
                        if !matches!(last_token, Token::Operator(_) | Token::OpenBrace) {
                            return Err(MathExpressionParserError::InvalidExpression { idx });
                        };
                    }
                }
                Token::CloseBrace => {
                    if braces.pop().is_none() {
                        return Err(MathExpressionParserError::InvalidExpression { idx });
                    }

                    let Some(last_token) = parsed_expression.expression.last() else {
                        return Err(MathExpressionParserError::InvalidExpression { idx });
                    };

                    if !matches!(last_token, Token::Digit(_) | Token::CloseBrace) {
                        return Err(MathExpressionParserError::InvalidExpression { idx });
                    }
                }
                Token::Digit(_) => {
                    if let Some(last_token) = parsed_expression.expression.last() {
                        if !matches!(last_token, Token::Operator(_) | Token::OpenBrace) {
                            return Err(MathExpressionParserError::InvalidExpression { idx });
                        };
                    }
                }
                Token::Operator(_) => {
                    let Some(last_token) = parsed_expression.expression.last() else {
                        return Err(MathExpressionParserError::InvalidExpression { idx });
                    };

                    if !matches!(last_token, Token::Digit(_) | Token::CloseBrace) {
                        return Err(MathExpressionParserError::InvalidExpression { idx });
                    }
                }
            }

            parsed_expression.expression.push(token);
        }

        if let Some(last_token) = parsed_expression.expression.last() {
            if matches!(last_token, Token::Operator(_) | Token::OpenBrace) {
                return Err(MathExpressionParserError::InvalidExpression {
                    idx: tokenizer.curr_index(),
                });
            }
        }

        if braces.is_empty() {
            Ok(parsed_expression)
        } else {
            Err(MathExpressionParserError::InvalidBraceConsequence {
                idx: *braces.last().unwrap(),
            })
        }
    }
}
