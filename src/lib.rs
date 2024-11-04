use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub vit);

pub mod ast;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_declaration() {
        let parser = vit::ProgramParser::new();
        
        assert!(parser.parse("let a; let b; let c = 24;").is_ok());
    }

    #[test]
    fn test_missing_semicolon() {
        let parser = vit::ProgramParser::new();

        assert!(parser.parse("let a").is_err());
        assert!(parser.parse("let a = 23").is_err());
    }


    #[test]
    fn test_invalid_assignment() {
        let parser = vit::ProgramParser::new();

        assert!(parser.parse("let a = ").is_err());
        assert!(parser.parse("a =").is_err());
        assert!(parser.parse("a = -").is_err());
    }

    #[test]
    fn test_invalid_id() {
        let parser = vit::ProgramParser::new();

        assert!(parser.parse("let 2a = 23;").is_err());
        assert!(parser.parse("let + = 23;").is_err());
        assert!(parser.parse("let = = 23;").is_err());
        assert!(parser.parse("let 24 = 23;").is_err());
    }

    #[test]
    fn test_keyword_id(){
        let parser = vit::ProgramParser::new();

        assert!(parser.parse("let let;").is_err());
        assert!(parser.parse("let read;").is_err());
    }

    #[test]
    fn test_valid_expression() {
        let parser = vit::ProgramParser::new();

        assert!(parser.parse("let a = 23 + 24;").is_ok());
        assert!(parser.parse("let a = 23 + 8 ^ 2 * 3;").is_ok());
    }

    #[test]
    fn test_precedence() {
        let parser = vit::ProgramParser::new();

        if let Ok(result) = parser.parse("let a = 23 + 8 ^ 2 * 3;") {
            if let ast::Statement::Declaration(id, expression) = result.get(0).unwrap() {
                assert_eq!(String::from("Some((23 + ((8 ^ 2) * 3)))"), format!("{:?}", expression));
                assert_eq!(String::from("a"), id.clone());
            }
        }
    }

    #[test]
    fn test_simple_if() {
        let parser = vit::ProgramParser::new();
        let result = parser.parse("if 2 == 2 { }");
        if let Ok(statement) = result {
            assert_eq!("[If((2 == 2), [], None)]", format!("{statement:?}"));
        }
    }

    fn test_complex_if() {
        let parser = vit::ProgramParser::new();
        let result = parser.parse("if a % 2 == 0 and (b > a or a == 4) {  }");
        if let Ok(statement) = result {
            assert_eq!("[If(((a % 2) and ((b > a) or (a == 4))), [], None)]", format!("{statement:?}"));
        }

        let result = parser.parse("if a % 2 == 0 and b > a or a == 4 {  }");
        if let Ok(statement) = result {
            assert_eq!("[If(((((a % 2) == 0) and (b > a)) or (a == 4))), [], None)]", format!("{statement:?}"));
        }
    }

    #[test]
    fn test_if_with_invalid_expression() {
        let parser = vit::ProgramParser::new();
        
        assert!(parser.parse("if 4 { }").is_err());
        assert!(parser.parse("if b { }").is_err());
    }
}