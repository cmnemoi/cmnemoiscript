use regex::Regex;

fn tokenize(source_code: &str) -> Vec<Token> {
    let mut source_code = source_code.split("").collect::<Vec<&str>>();

    let mut tokens: Vec<Token> = Vec::new();

    while !source_code.is_empty() {
        match source_code[0] {
            "=" => {
                tokens.push(Token {
                    kind: TokenType::Equals,
                    value: source_code.remove(0).to_string(),
                });
            }
            "*" => {
                let number_of_characters_to_remove = if source_code[1] != "*" { 1 } else { 2 };
                tokens.push(Token {
                    kind: TokenType::BinaryOperation,
                    value: source_code
                        .drain(..number_of_characters_to_remove)
                        .collect::<String>(),
                });
            }
            character if is_single_character_binary_operation_token(character) => {
                tokens.push(Token {
                    kind: TokenType::BinaryOperation,
                    value: source_code.remove(0).to_string(),
                });
            }
            character if is_integer(character) => {
                let mut number = "".to_string();
                while !source_code.is_empty() && is_integer(source_code[0]) {
                    number = format!("{}{}", number, source_code.remove(0));
                }

                tokens.push(Token {
                    kind: TokenType::Number,
                    value: number,
                });
            }
            character if is_alphanumeric(character) => {
                let mut variable = "".to_string();
                while !source_code.is_empty() && is_alphanumeric(source_code[0]) {
                    variable = format!("{}{}", variable, source_code.remove(0));
                }

                tokens.push(Token {
                    kind: TokenType::Variable,
                    value: variable,
                });
            }
            _ => {
                // Handle unrecognized token
                source_code.remove(0);
            }
        }
    }

    tokens
}

fn is_single_character_binary_operation_token(character: &str) -> bool {
    character == "+" || character == "-" || character == "*" || character == "/" || character == "%"
}

fn is_alphanumeric(character: &str) -> bool {
    Regex::new(r"^[a-zA-Z0-9]+$").unwrap().is_match(character)
}

fn is_integer(character: &str) -> bool {
    Regex::new(r"^[0-9]$").unwrap().is_match(character)
}

#[derive(Debug, PartialEq)]
enum TokenType {
    BinaryOperation,
    Equals,
    Number,
    Variable,
}

#[derive(Debug, PartialEq)]
struct Token {
    kind: TokenType,
    value: String,
}
#[cfg(test)]
mod tests {
    use crate::lexer::{tokenize, Token, TokenType};

    #[test]
    fn should_tokenize_x_equals_42() {
        let expected_tokens = vec![
            Token {
                kind: TokenType::Variable,
                value: "x".to_string(),
            },
            Token {
                kind: TokenType::Equals,
                value: "=".to_string(),
            },
            Token {
                kind: TokenType::Number,
                value: "42".to_string(),
            },
        ];
        let actual_tokens = tokenize("x = 42");

        assert_eq!(actual_tokens, expected_tokens)
    }

    #[test]
    fn should_tokenize_x_equals_43() {
        let expected_tokens = vec![
            Token {
                kind: TokenType::Variable,
                value: "x".to_string(),
            },
            Token {
                kind: TokenType::Equals,
                value: "=".to_string(),
            },
            Token {
                kind: TokenType::Number,
                value: "43".to_string(),
            },
        ];
        let actual_tokens = tokenize("x = 43");

        assert_eq!(actual_tokens, expected_tokens)
    }

    #[test]
    fn should_tokenize_y_equals_42() {
        let expected_tokens = vec![
            Token {
                kind: TokenType::Variable,
                value: "y".to_string(),
            },
            Token {
                kind: TokenType::Equals,
                value: "=".to_string(),
            },
            Token {
                kind: TokenType::Number,
                value: "42".to_string(),
            },
        ];
        let actual_tokens = tokenize("y = 42");

        assert_eq!(actual_tokens, expected_tokens)
    }

    #[test]
    fn should_tokenize_xy_equals_42() {
        let expected_tokens = vec![
            Token {
                kind: TokenType::Variable,
                value: "xy".to_string(),
            },
            Token {
                kind: TokenType::Equals,
                value: "=".to_string(),
            },
            Token {
                kind: TokenType::Number,
                value: "42".to_string(),
            },
        ];
        let actual_tokens = tokenize("xy = 42");

        assert_eq!(actual_tokens, expected_tokens)
    }

    #[test]
    fn should_tokenize_x_plus_y_equals_42() {
        let expected_tokens = vec![
            Token {
                kind: TokenType::Variable,
                value: "x".to_string(),
            },
            Token {
                kind: TokenType::BinaryOperation,
                value: "+".to_string(),
            },
            Token {
                kind: TokenType::Variable,
                value: "y".to_string(),
            },
            Token {
                kind: TokenType::Equals,
                value: "=".to_string(),
            },
            Token {
                kind: TokenType::Number,
                value: "42".to_string(),
            },
        ];
        let actual_tokens = tokenize("x + y = 42");

        assert_eq!(actual_tokens, expected_tokens)
    }

    #[test]
    fn should_tokenize_x_minus_y_equals_42() {
        let expected_tokens = vec![
            Token {
                kind: TokenType::Variable,
                value: "x".to_string(),
            },
            Token {
                kind: TokenType::BinaryOperation,
                value: "-".to_string(),
            },
            Token {
                kind: TokenType::Variable,
                value: "y".to_string(),
            },
            Token {
                kind: TokenType::Equals,
                value: "=".to_string(),
            },
            Token {
                kind: TokenType::Number,
                value: "42".to_string(),
            },
        ];
        let actual_tokens = tokenize("x - y = 42");

        assert_eq!(actual_tokens, expected_tokens)
    }

    #[test]
    fn should_tokenize_x_times_y_equals_42() {
        let expected_tokens = vec![
            Token {
                kind: TokenType::Variable,
                value: "x".to_string(),
            },
            Token {
                kind: TokenType::BinaryOperation,
                value: "*".to_string(),
            },
            Token {
                kind: TokenType::Variable,
                value: "y".to_string(),
            },
            Token {
                kind: TokenType::Equals,
                value: "=".to_string(),
            },
            Token {
                kind: TokenType::Number,
                value: "42".to_string(),
            },
        ];
        let actual_tokens = tokenize("x * y = 42");

        assert_eq!(actual_tokens, expected_tokens)
    }

    #[test]
    fn should_tokenize_x_divided_by_y_equals_42() {
        let expected_tokens = vec![
            Token {
                kind: TokenType::Variable,
                value: "x".to_string(),
            },
            Token {
                kind: TokenType::BinaryOperation,
                value: "/".to_string(),
            },
            Token {
                kind: TokenType::Variable,
                value: "y".to_string(),
            },
            Token {
                kind: TokenType::Equals,
                value: "=".to_string(),
            },
            Token {
                kind: TokenType::Number,
                value: "42".to_string(),
            },
        ];
        let actual_tokens = tokenize("x / y = 42");

        assert_eq!(actual_tokens, expected_tokens)
    }

    #[test]
    fn should_tokenize_x_modulo_y_equals_42() {
        let expected_tokens = vec![
            Token {
                kind: TokenType::Variable,
                value: "x".to_string(),
            },
            Token {
                kind: TokenType::BinaryOperation,
                value: "%".to_string(),
            },
            Token {
                kind: TokenType::Variable,
                value: "y".to_string(),
            },
            Token {
                kind: TokenType::Equals,
                value: "=".to_string(),
            },
            Token {
                kind: TokenType::Number,
                value: "42".to_string(),
            },
        ];
        let actual_tokens = tokenize("x % y = 42");

        assert_eq!(actual_tokens, expected_tokens)
    }

    #[test]
    fn should_tokenize_x_power_y_equals_42() {
        let expected_tokens = vec![
            Token {
                kind: TokenType::Variable,
                value: "x".to_string(),
            },
            Token {
                kind: TokenType::BinaryOperation,
                value: "**".to_string(),
            },
            Token {
                kind: TokenType::Variable,
                value: "y".to_string(),
            },
            Token {
                kind: TokenType::Equals,
                value: "=".to_string(),
            },
            Token {
                kind: TokenType::Number,
                value: "42".to_string(),
            },
        ];
        let actual_tokens = tokenize("x ** y = 42");

        assert_eq!(actual_tokens, expected_tokens)
    }

    #[test]
    fn should_tokenize_x_minus_y_equals_42_without_spaces() {
        let expected_tokens = vec![
            Token {
                kind: TokenType::Variable,
                value: "x".to_string(),
            },
            Token {
                kind: TokenType::BinaryOperation,
                value: "-".to_string(),
            },
            Token {
                kind: TokenType::Variable,
                value: "y".to_string(),
            },
            Token {
                kind: TokenType::Equals,
                value: "=".to_string(),
            },
            Token {
                kind: TokenType::Number,
                value: "42".to_string(),
            },
        ];
        let actual_tokens = tokenize("x-y=42");

        assert_eq!(actual_tokens, expected_tokens)
    }

    #[test]
    fn should_tokenize_price1_equals_42() {
        let expected_tokens = vec![
            Token {
                kind: TokenType::Variable,
                value: "price1".to_string(),
            },
            Token {
                kind: TokenType::Equals,
                value: "=".to_string(),
            },
            Token {
                kind: TokenType::Number,
                value: "42".to_string(),
            },
        ];
        let actual_tokens = tokenize("price1 = 42");

        assert_eq!(actual_tokens, expected_tokens)
    }
}
