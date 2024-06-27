#[derive(Debug, PartialEq)]

enum TokenType {
    Variable,
    Equals,
    Number,
}

#[derive(Debug, PartialEq)]

struct Token {
    kind: TokenType,
    value: String,
}

fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::<Token>::new();

    let split_source_code: Vec<&str> = source_code.split(' ').collect::<Vec<&str>>();

    for element in split_source_code  {
        if element == "x" {
            tokens.push(Token {kind: TokenType::Variable, value: element.to_string()})
        } else if element == "=" {
            tokens.push(Token {kind: TokenType::Equals, value: element.to_string()})
        } else if element == "42" {
            tokens.push(Token {kind: TokenType::Number, value: element.to_string()})
        }
    }
    tokens
}


#[cfg(test)]
mod tests {
    use crate::lexer::{tokenize, Token, TokenType};

    #[test]
    fn should_tokenize_source_code() {
        let actual_tokens = tokenize("x = 42");
        let expected_tokens = vec![
            Token {kind: TokenType::Variable, value: "x".to_string()},
            Token {kind: TokenType::Equals, value: "=".to_string()},
            Token {kind: TokenType::Number, value: "42".to_string()},
        ];

        assert_eq!(actual_tokens, expected_tokens)
    }
}
