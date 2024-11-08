use std::collections::HashMap;

use crate::ast::{Expr, Statement};

mod expressions;

// type Address = u32;
// type Label = String;

struct Variable {
    address: u32,
    initialized: bool,
}

// Generates p-code from the AST created by the parser.
pub struct Translator {
    stack: Vec<HashMap<String, Variable>>,
    current_address: u32, // When the next value is stored, it will go in this address.,
    label_count: u32,
    labels: Vec<u32>,
}

// pub fn build() -> Translator {
//     let root_frame = Frame {
//         symbol_table: HashMap::new(),
//         is_root_frame: true,
//     };

//     Translator {
//         stack: vec![root_frame],
//         current_address: 0,
//     }
// }

impl Translator {
    pub fn new() -> Self {
        let stack = vec![HashMap::new()];
        Translator {
            stack,
            current_address: 0,
            label_count: 0,
            labels: vec![],
        }
    }
    
    pub fn run(&mut self, program: Vec<Statement>) -> Result<String, String> {
        let mut result = String::new();

        for statement in program {
            result.push_str(&self.parse_statement(statement)?);
        }

        Ok(result)
    }

    fn parse_statement(&mut self, statement: Statement) -> Result<String, String> {
        match statement {
            Statement::Declaration(id, expr) => self.declare(id, expr),
            Statement::Assignment(id, expr) => self.assign(id, expr),
            Statement::Read(id) => self.read(id),
            Statement::WriteId(id) => self.write(id),
            Statement::WriteLiteral(string) => self.write_string(string),
            Statement::If(predicate, block, else_block) => {
                self.if_statement(predicate, block, else_block)
            },
            Statement::Loop(block) => self.u_loop(block),
            Statement::Break => self.break_loop(),
            _ => Err("Not implemented yet.".to_string()),
        }
    }

    fn if_statement(&mut self, predicate: Box<Expr>,
        if_block: Vec<Statement>,
        else_block: Option<Vec<Statement>>) -> Result<String, String> {

           Ok(String::new()) 
    }

    fn break_loop(&mut self) -> Result<String, String> {
        if self.labels.is_empty() {
            return Err("break not inside a loop.".to_string());
        }
        Ok(format!("ujp E{}\n", self.labels.last().unwrap()))
    }

    fn u_loop(&mut self, block: Vec<Statement>) -> Result<String, String> {
        let mut result = String::new();

        let label = self.label_count;
        self.labels.push(label);
        self.label_count += 1;
        self.push_scope();

        result.push_str(&format!("L{label}:\n"));
        
        for statement in block {
            result.push_str(&self.parse_statement(statement)?);
        }
    
        result.push_str(&format!("ujp L{label}\nE{label}:\n")); // This label allows the program to break from the loop.
        self.labels.pop();
        self.pop_scope();
        Ok(result)
    }
    
    fn read(&mut self, id: String) -> Result<String, String> {
        let address = Self::get_address(&self.stack, &id)?.0;
        Ok(format!("lda #{}\nrd\nsto\n", address))
    }

    fn write(&mut self, id: String) -> Result<String, String> {
        let variable = Self::get_address(&self.stack, &id)?;

        if !variable.1 {
            return Err("unitialized variable.".to_string());
        }

        Ok(format!("lod #{}\nwri\n", variable.0))
    }

    fn write_string(&mut self, string: String) -> Result<String, String> {
        Ok(format!("ldc \"{}\"\nwri\n", string.replace("'", "")))
    }

    fn assign(&mut self, id: String, expr: Box<Expr>) -> Result<String, String> {
        let address = Self::get_address(&self.stack, &id)?.0;

        let mut result = format!("lda #{address}\n");
        Self::parse_expression(&self.stack, *expr, &mut result)?;

        result.push_str("sto\n");
        Ok(result)
    }

    fn declare(&mut self, id: String, e: Option<Box<Expr>>) -> Result<String, String> {
        if self.stack.is_empty() {
            self.stack.push(HashMap::new());
        }
        
        if self.stack.last().unwrap().contains_key(&id) { return Err(format!("variable already declared: {}.", id)); }
        
        let mut variable = Variable {address: self.current_address, initialized: e.is_some()};
        
        let mut result = String::new();
        
        if let Some(expr) = e {
            result.push_str(&format!("lda #{}\n", self.current_address));
            let _ = Self::parse_expression(&self.stack, *expr, &mut result)?;
            result.push_str("sto\n");
            variable.initialized = true;
        }
        
        self.stack.last_mut().unwrap().insert(id, variable);
        self.current_address += 1;
        Ok(result)
    }
    
    fn push_scope(&mut self) -> () {
        self.stack.push(HashMap::new());
    }

    fn pop_scope(&mut self) -> () {
        if let Some(scope) = self.stack.pop() {
            self.current_address -= scope.len() as u32;
        } else {
            panic!("the stack is empty.");
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use lalrpop_util::lalrpop_mod;
    
    lalrpop_mod!(pub vit_grammar);
    
    
    #[test]
    fn valid_declaration() {
        let mut translator = Translator::new();
        let statement = Statement::Declaration("a".to_string(), None);
        
        let result = translator.parse_statement(statement);
        assert!(result.unwrap().is_empty());
        assert_eq!(translator.current_address, 1);
        assert!(translator.stack.get(0).unwrap().contains_key("a"));
    }
    
    #[test]
    fn valid_declaration_with_assignment() {
        let mut translator = Translator::new();
        
        let _ = translator.parse_statement(Statement::Declaration("a".to_string(),
        Some(vit_grammar::ExprParser::new().parse("24").unwrap())));
        
        let statement = Statement::Declaration("b".to_string(),
        Some(vit_grammar::ExprParser::new().parse("a * 2 + 1").unwrap())
    );
    
    let result = translator.parse_statement(statement);
    assert_eq!(result.unwrap(), "lda #1\nlod #0\nldc 2\nmul\nldc 1\nadd\nsto\n");
    assert_eq!(translator.current_address, 2);
    assert!(translator.stack.get(0).unwrap().contains_key("b"));
    }

    #[test]
    fn declaration_with_shadowing() {
        let mut translator = Translator::new();
        
        let _ = translator.parse_statement(Statement::Declaration("a".to_string(),
        Some(vit_grammar::ExprParser::new().parse("24").unwrap())));
        
        let statement = Statement::Declaration("a".to_string(),
        Some(vit_grammar::ExprParser::new().parse("4").unwrap())
    );

    let result = translator.parse_statement(statement);
    assert!(result.is_err());
    }

    #[test]
    fn declaration_inside_inner_scope() {
        let mut translator = Translator::new();
        
        let _ = translator.parse_statement(Statement::Declaration("a".to_string(),
        Some(vit_grammar::ExprParser::new().parse("24").unwrap())));
        
        translator.stack.push(HashMap::new());
        
        let statement = Statement::Declaration("b".to_string(),
        Some(vit_grammar::ExprParser::new().parse("a * 2").unwrap())
    );

    let result = translator.parse_statement(statement);
    assert_eq!(result.unwrap(), "lda #1\nlod #0\nldc 2\nmul\nsto\n");
    assert_eq!(translator.current_address, 2);
    assert!(translator.stack.last().unwrap().contains_key("b"));
    }

    #[test]
    fn use_after_scope() {
        let mut translator = Translator::new();
        
        translator.stack.push(HashMap::new());
        
        let statement = Statement::Declaration("b".to_string(),
        Some(vit_grammar::ExprParser::new().parse("2").unwrap())
    );

    let _ = translator.parse_statement(statement);

    translator.stack.pop();

    let statement = Statement::Declaration("a".to_string(),
    Some(vit_grammar::ExprParser::new().parse("b + 2").unwrap())
    );

    let result = translator.parse_statement(statement);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("undeclared variable"));
    }

    #[test]
    fn assign_to_undefined_variable() {
        let mut translator = Translator::new();

        let result = translator.parse_statement(Statement::Assignment("a".to_string(),
            vit_grammar::ExprParser::new().parse("24").unwrap())
        );

        assert!(result.is_err());
    }

    #[test]
    fn assign_to_variable() {
        let mut translator = Translator::new();

        let _ = translator.parse_statement(
            Statement::Declaration("age".to_string(), None)
        );

        let result = translator.parse_statement(
            Statement::Assignment("age".to_string(),
            vit_grammar::ExprParser::new().parse("24").unwrap())
        ).unwrap();

        assert_eq!(result, "lda #0\nldc 24\nsto\n");
    }

    #[test]
    fn assign_expression_to_variable() {
        let mut translator = Translator::new();
        let parser = vit_grammar::ExprParser::new();

        let _ = translator.parse_statement(
            Statement::Declaration("average".to_string(), None)
        );

        let _ = translator.parse_statement(
            Statement::Declaration("n1".to_string(),
            Some(parser.parse("7.8").unwrap()))
        );

        let _ = translator.parse_statement(
            Statement::Declaration("n2".to_string(),
            Some(parser.parse("9.0").unwrap()))
        );

        let result = translator.parse_statement(
            Statement::Assignment("average".to_string(),
            parser.parse("(n1 + n2) / 2").unwrap())
        ).unwrap();

        assert_eq!(result, "lda #0\nlod #1\nlod #2\nadd\nldc 2\ndiv\nsto\n");
        assert_eq!(translator.current_address, 3);
    }

    #[test]
    fn read_to_variable() {
        let mut translator = Translator::new();
        let _ = translator.parse_statement(Statement::Declaration("age".to_string(), None));

        let result = translator.parse_statement(
            Statement::Read("age".to_string())
        );

        assert_eq!(result.unwrap(), "lda #0\nrd\nsto\n");
    }

    #[test]
    fn read_to_undeclared_variable() {
        let mut translator = Translator::new();

        let result = translator.parse_statement(
            Statement::Read("age".to_string())
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("undeclared variable"));
    }

    #[test]
    fn write_unitialized_variable() {
        let mut translator = Translator::new();

        let _ = translator.parse_statement(
            Statement::Declaration("a".to_string(), None)
        );

        let result = translator.write("a".to_string());

        println!("{result:?}");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unitialized"));
    }

    #[test]
    fn write_valid_variable() {
        let mut translator = Translator::new();

        let _ = translator.parse_statement(Statement::Declaration(
            "a".to_string(),
            Some(vit_grammar::ExprParser::new().parse("-50").unwrap()),
        ));

        let result = translator.write("a".to_string());

        println!("{result:?}");
        assert_eq!(result.unwrap(), "lod #0\nwri\n");
    }

    #[test]
    fn write_string() {
        let mut translator = Translator::new();

        let result = translator.write_string("hello, world!\\n".to_string());

        assert_eq!(result.unwrap(), "ldc \"hello, world!\\n\"\nwri\n");
    }

    #[test]
    fn infinite_loop() {
        let program = vit_grammar::ProgramParser::new()
            .parse("\
            let a = 24;
            loop {
                a = a + 1;
                write a;
            }").unwrap();

        let mut translator = Translator::new();
        
        let result = translator.run(program);
        // println!("{}", result);
        // assert!(false);
    
        assert_eq!(result.unwrap(), "lda #0\nldc 24\nsto\nL0:\nlda #0\nlod #0\nldc 1\nadd\nsto\nlod #0\nwri\nujp L0\nE0:\n");
    }

    #[test]
    fn test_break() {
        let program = vit_grammar::ProgramParser::new()
            .parse("loop {
                write 'Loop 0.\\n';
                loop {
                    write 'Loop 1.\\n';
                    loop {
                        write 'Loop 2.\\n';
                        break;
                    }
                    break;
                }
                break;
            }").unwrap();

        let mut translator = Translator::new();
        
        let result = translator.run(program);
    
        assert_eq!(result.unwrap(), "L0:\nldc \"Loop 0.\\n\"\nwri\nL1:\nldc \"Loop 1.\\n\"\nwri\nL2:\nldc \"Loop 2.\\n\"\nwri\nujp E2\nujp L2\nE2:\nujp E1\nujp L1\nE1:\nujp E0\nujp L0\nE0:\n");
    }

    #[test]
    fn test_scope() {
        let program = vit_grammar::ProgramParser::new()
            .parse("loop {
                let a = 2;
                break;
            }
            write a;").unwrap();

        let mut translator = Translator::new();
        
        let result = translator.run(program);
    
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("undeclared"));
        assert_eq!(translator.current_address, 0);
    }
}