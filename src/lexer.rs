fn tokenize(source_code: &str) -> Vec<Token> {
    source_code
        .split(' ')
        .map(get_token_from)
        .collect::<Vec<Token>>()
}

fn get_token_from(character: &str) -> Token {
    match character {
        "=" => Token {
            kind: TokenType::Equals,
            value: character.to_string(),
        },
        _ if is_binary_operation_token(character) => Token {
            kind: TokenType::BinaryOperation,
            value: character.to_string(),
        },
        _ if character.parse::<f64>().is_ok() => Token {
            kind: TokenType::Number,
            value: character.to_string(),
        },
        _ if character.is_ascii() => Token {
            kind: TokenType::Variable,
            value: character.to_string(),
        },
        _ => panic!("Syntax error: invalid token {}", character),
    }
}

fn is_binary_operation_token(character: &str) -> bool {
    character == "+"
        || character == "-"
        || character == "*"
        || character == "/"
        || character == "%"
        || character == "**"
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
}
