use std::{collections::HashMap};

use crate::ast::{Expr, Opcode, Statement};

type Address = u32;
type Label = String;

// Generates p-code from the AST created by the parser.
enum DataType {
    String(String),
    Float(f32),
    Integer(i32),
}

struct Frame {
    symbol_table: HashMap<String, u32>,
    is_root_frame: bool,
}

pub struct Translator {
    stack: Vec<Frame>,
    current_address: u32, // When the next value is stored, it will go in this address.,
}

impl Translator {
    pub fn build(ast: Vec<Statement>) -> Translator {
        let mut root_frame = Frame {
            symbol_table: HashMap::new(),
            is_root_frame: true,
        };
        
        Translator {
            stack: vec![root_frame],
            current_address: 0,
        }
    }

    pub fn run(mut self, ast: Vec<Statement>) -> () {

        let mut result = String::new();
    }

    fn parse_op(op: Opcode) -> &'static str {
        match op {
            Opcode::Add => "add\n",
            Opcode::Sub => "sub\n",
            Opcode::Mul => "mul\n",
            Opcode::Div => "div\n",
            _ => "",
        }
    }
    
    fn parse_expression(symbol_table: &HashMap<String, u32>, expr: Expr, result: &mut String) -> Result<(), String> {
        
        match expr {
            Expr::Number(sign, num) => {
                result.push_str(&format!("ldc {}{}\n", if sign { "-"} else { ""}, num));
            },
            Expr::Op(l, op, r) => {
                Self::parse_expression(symbol_table, *l, result)?;
                Self::parse_expression(symbol_table, *r, result)?;
                result.push_str(Self::parse_op(op));
    
            },
            Expr::Id(id) => {
                let address = symbol_table.get(&id);
                if address.is_none() {
                    return Err(format!("Undefined symbol: {id}."));
                }
                result.push_str(&format!("lod #{}\n", address.unwrap()));
            },
            _ => (),
        }
    
        Ok(())
    }

    fn add_identifier(&mut self, id: String) -> () {
        self.stack.last_mut().unwrap().symbol_table.insert(id, self.current_address);
        self.current_address += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use super::*;

    use lalrpop_util::{lalrpop_mod, lexer::Token, ParseError};

    lalrpop_mod!(pub vit_grammar);

    #[test]
    fn valid_expression() {
        if let Ok(expr) = vit_grammar::ExprParser::new().parse("2 + 3 * 4 - 3") {
            let mut result = String::new();
            let table: HashMap<String, u32> = HashMap::new();
            let _ = Translator::parse_expression(&table, *expr, &mut result);
            assert_eq!(result, "ldc 2\nldc 3\nldc 4\nmul\nadd\nldc 3\nsub\n");
        }
    }

    #[test]
    fn expression_with_undefined_id() {
        let expr = vit_grammar::ExprParser::new().parse("2 + 3 * a - 3").unwrap();
        
        let mut result = String::new();
            
        let table: HashMap<String, u32> = HashMap::new();
    
        assert!(Translator::parse_expression(&table, *expr, &mut result).is_err());
    }

    #[test]
    fn valid_expression_with_id() {
        let expr = vit_grammar::ExprParser::new().parse("(7 * (start + 2) - 2) + 2 / a").unwrap();
        
        let mut result = String::new();
            
            
        let mut table: HashMap<String, u32> = HashMap::new();
        table.insert("a".to_string(), 0);
        table.insert("start".to_string(), 1);
    
        Translator::parse_expression(&table, *expr, &mut result);
        assert_eq!(result, "ldc 7\nlod #1\nldc 2\nadd\nmul\nldc 2\nsub\nldc 2\nlod #0\ndiv\nadd\n");
    }

    #[test]
    fn valid_predicate() {
        
        let expr = vit_grammar::PredicateParser::new().parse("2 + 3 * a > b / 2").unwrap();

        let mut result = String::new();
            
            
        let mut table: HashMap<String, u32> = HashMap::new();
        table.insert("a".to_string(), 0);
        table.insert("start".to_string(), 1);
    
        assert!(Translator::parse_expression(&table, *expr, &mut result).is_ok());
        assert_eq!(result, "ldc 7\nlod #1\nldc 2\nadd\nmul\nldc 2\nsub\nldc 2\nlod #0\ndiv\nadd\n");
    }
}