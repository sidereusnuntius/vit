use std::collections::HashMap;

use crate::ast::Statement;
use lalrpop_util::{lalrpop_mod, ParseError};

lalrpop_mod!(pub vit_grammar);

pub struct Parser {
    parser: vit_grammar::ProgramParser,
    map: HashMap<String, String>,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            parser: vit_grammar::ProgramParser::new(),
            map: {
                let mut map = HashMap::new();
                map.insert(
                    "r#\"[a-zA-z][a-zA-z0-9_]*\"#".to_string(),
                    "an identifier".to_string(),
                );
                map.insert("r#\"'.*'\"#".to_string(), "a string literal".to_string());
                map
            },
        }
    }

    pub fn parse(&self, source: &str) -> Result<Vec<Statement>, String> {
        match self.parser.parse(source) {
            Err(error) => match error {
                ParseError::InvalidToken { location } => {
                    Err(format!("invalid token at {location}"))
                }
                ParseError::UnrecognizedToken { token, expected } => Err(format!(
                    "expected {:?}, found {}",
                    expected
                        .iter()
                        .map(|p| self.map.get(p).unwrap_or(p).clone())
                        .collect::<Vec<String>>(),
                    token.1,
                )),
                ParseError::UnrecognizedEof { location, .. } => {
                    Err(format!("unexpected EoF at location {location}"))
                }
                ParseError::User { error } => Err(error.to_string()),
                ParseError::ExtraToken { token } => Err(format!("extra token: {}", token.1)),
            },
            Ok(program) => Ok(program),
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_declaration() {
        let parser = Parser::new();

        assert!(parser.parse("let a; let b; let c = 24;").is_ok());
    }

    #[test]
    fn test_missing_semicolon() {
        let parser = Parser::new();

        assert!(parser.parse("let a").is_err());
        assert!(parser.parse("let a = 23").is_err());
    }

    #[test]
    fn test_invalid_assignment() {
        let parser = Parser::new();

        assert!(parser.parse("let a = ").is_err());
        assert!(parser.parse("a =").is_err());
        assert!(parser.parse("a = -").is_err());
    }

    #[test]
    fn test_invalid_id() {
        let parser = Parser::new();

        assert!(parser.parse("let 2a = 23;").is_err());
        assert!(parser.parse("let + = 23;").is_err());
        assert!(parser.parse("let = = 23;").is_err());
        assert!(parser.parse("let 24 = 23;").is_err());
    }

    #[test]
    fn test_keyword_id() {
        let parser = Parser::new();

        assert!(parser.parse("let let;").is_err());
        assert!(parser.parse("let read;").is_err());
    }

    #[test]
    fn test_valid_expression() {
        let parser = Parser::new();

        assert!(parser.parse("let a = 23 + 24;").is_ok());
        assert!(parser.parse("let a = 23 + 8 ^ 2 * 3;").is_ok());
    }

    #[test]
    fn test_precedence() {
        let parser = Parser::new();

        if let Ok(result) = parser.parse("let a = 23 + 8 ^ 2 * 3;") {
            if let Statement::Declaration(id, expression) = result.get(0).unwrap() {
                assert_eq!(
                    String::from("Some((23 + ((8 ^ 2) * 3)))"),
                    format!("{:?}", expression)
                );
                assert_eq!(String::from("a"), id.clone());
            }
        }
    }

    #[test]
    fn test_simple_if() {
        let parser = Parser::new();
        let result = parser.parse("if 2 == 2 { }");
        if let Ok(statement) = result {
            assert_eq!("[If((2 == 2), [], None)]", format!("{statement:?}"));
        }
    }

    fn test_complex_if() {
        let parser = Parser::new();
        let result = parser.parse("if a % 2 == 0 and (b > a or a == 4) {  }");
        if let Ok(statement) = result {
            assert_eq!(
                "[If(((a % 2) and ((b > a) or (a == 4))), [], None)]",
                format!("{statement:?}")
            );
        }

        let result = parser.parse("if a % 2 == 0 and b > a or a == 4 {  }");
        if let Ok(statement) = result {
            assert_eq!(
                "[If(((((a % 2) == 0) and (b > a)) or (a == 4))), [], None)]",
                format!("{statement:?}")
            );
        }
    }

    #[test]
    fn test_if_with_invalid_expression() {
        let parser = Parser::new();

        assert!(parser.parse("if 4 { }").is_err());
        assert!(parser.parse("if b { }").is_err());
    }

    #[test]
    fn test_do_until() {
        let parser = Parser::new();
        assert!(parser
            .parse(
                "\
    let a = 0;
    loop {
        write a;
        a = a + 1;
        if a == 10 {
            break;
        }
    }
    write 'END OF THE PROGRAM\\n';"
            )
            .is_ok());
    }
}
