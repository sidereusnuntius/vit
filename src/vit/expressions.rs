use std::collections::HashMap;

use crate::ast::{Expr, Opcode};

use super::{State, Variable};

impl State {
    pub(super) fn get_address<'a>(
        stack: &'a mut Vec<HashMap<String, Variable>>,
        id: &String,
    ) -> Result<&'a mut Variable, String> {
        for scope in stack.iter_mut().rev() {
            if let Some(var) = scope.get_mut(id) {
                return Ok(var);
            }
        }
        Err(format!("undeclared variable: {}.", id))
    }

    pub(super) fn parse_expression(
        stack: &mut Vec<HashMap<String, Variable>>,
        expr: Expr,
        result: &mut String,
    ) -> Result<(), String> {
        match expr {
            Expr::Number(sign, num) => {
                result.push_str(&format!("ldc {}{}\n", if sign { "-" } else { "" }, num));
            }
            Expr::Op(l, op, r) => {
                if op == Opcode::Mod {
                    let mut left_expression = String::new();
                    let mut right_expression = String::new();

                    Self::parse_expression(stack, *l, &mut left_expression)?;
                    Self::parse_expression(stack, *r, &mut right_expression)?;

                    result.push_str(&left_expression);
                    result.push_str(&left_expression);
                    result.push_str(&right_expression);
                    result.push_str("div\nto int\n");
                    result.push_str(&right_expression);
                    result.push_str("mul\nsub\n");
                } else {
                    Self::parse_expression(stack, *l, result)?;
                    Self::parse_expression(stack, *r, result)?;
                    result.push_str(Self::parse_op(op));
                }
            }
            Expr::Id(id) => {
                let var = Self::get_address(stack, &id)?;
                if !var.initialized {
                    return Err(format!("uninitialized variable: {id}."));
                }
                result.push_str(&format!(
                    "lod #{}\n",
                    var.address,
                ));
            },
            _ => (),
        }

        Ok(())
    }

    pub(super) fn parse_op(op: Opcode) -> &'static str {
        match op {
            Opcode::Add => "add\n",
            Opcode::Sub => "sub\n",
            Opcode::Mul => "mul\n",
            Opcode::Div => "div\n",
            Opcode::Eq => "equ\n",
            Opcode::Neq => "neq\n",
            Opcode::Grt => "grt\n",
            Opcode::Geq => "gte\n",
            Opcode::Let => "let\n",
            Opcode::Leq => "lte\n",
            Opcode::And => "and\n",
            Opcode::Or => "or\n",
            _ => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use lalrpop_util::lalrpop_mod;

    lalrpop_mod!(pub vit_grammar);

    #[test]
    fn mod_operator() {
        let expr = vit_grammar::ExprParser::new().parse("a % 2").unwrap();
        let mut result = String::new();
        let mut stack: Vec<HashMap<String, Variable>> = vec![];

        let mut scope = HashMap::new();
        scope.insert(
            "a".to_string(),
            Variable {
                address: 0,
                initialized: true,
            },
        );
        stack.push(scope);

        let _ = State::parse_expression(&mut stack, *expr, &mut result);
        assert_eq!(
            result,
            "lod #0\nlod #0\nldc 2\ndiv\nto int\nldc 2\nmul\nsub\n"
        );
    }

    #[test]
    fn valid_expression() {
        if let Ok(expr) = vit_grammar::ExprParser::new().parse("2 + 3 * 4 - 3") {
            let mut result = String::new();
            let mut stack: Vec<HashMap<String, Variable>> = vec![];
            let _ = State::parse_expression(&mut stack, *expr, &mut result);
            assert_eq!(result, "ldc 2\nldc 3\nldc 4\nmul\nadd\nldc 3\nsub\n");
        }
    }

    #[test]
    fn expression_with_undefined_id() {
        let expr = vit_grammar::ExprParser::new()
            .parse("2 + 3 * a - 3")
            .unwrap();

        let mut result = String::new();

        let mut stack: Vec<HashMap<String, Variable>> = vec![];
        stack.push(HashMap::new());

        assert!(State::parse_expression(&mut stack, *expr, &mut result).is_err());
    }

    #[test]
    fn valid_expression_with_id() {
        let expr = vit_grammar::ExprParser::new()
            .parse("(7 * (start + 2) - 2) + 2 / a")
            .unwrap();

        let mut result = String::new();

        let mut stack: Vec<HashMap<String, Variable>> = vec![];
        let mut table: HashMap<String, Variable> = HashMap::new();

        table.insert(
            "a".to_string(),
            Variable {
                address: 0,
                initialized: true,
            },
        );
        table.insert(
            "start".to_string(),
            Variable {
                address: 1,
                initialized: true,
            },
        );

        stack.push(table);

        let _ = State::parse_expression(&mut stack, *expr, &mut result);
        assert_eq!(
            result,
            "ldc 7\nlod #1\nldc 2\nadd\nmul\nldc 2\nsub\nldc 2\nlod #0\ndiv\nadd\n"
        );
    }

    #[test]
    fn valid_expression_with_multiple_scopes() {
        let expr = vit_grammar::ExprParser::new()
            .parse("2 + a - 3 * b")
            .unwrap();

        let mut result = String::new();

        let mut stack: Vec<HashMap<String, Variable>> = vec![];

        let mut table: HashMap<String, Variable> = HashMap::new();

        table.insert(
            "a".to_string(),
            Variable {
                address: 0,
                initialized: true,
            },
        );
        table.insert(
            "b".to_string(),
            Variable {
                address: 1,
                initialized: true,
            },
        );

        stack.push(table);

        let mut table: HashMap<String, Variable> = HashMap::new();
        table.insert(
            "b".to_string(),
            Variable {
                address: 2,
                initialized: true,
            },
        );

        stack.push(table);

        let _ = State::parse_expression(&mut stack, *expr, &mut result);
        assert_eq!(result, "ldc 2\nlod #0\nadd\nldc 3\nlod #2\nmul\nsub\n");
    }

    #[test]
    fn valid_simple_predicate() {
        let expr = vit_grammar::PredicateParser::new()
            .parse("2 + 3 * a > b / 2")
            .unwrap();

        let mut result = String::new();

        let mut table: HashMap<String, Variable> = HashMap::new();
        table.insert(
            "a".to_string(),
            Variable {
                address: 0,
                initialized: true,
            },
        );
        table.insert(
            "b".to_string(),
            Variable {
                address: 1,
                initialized: true,
            },
        );

        let mut stack: Vec<HashMap<String, Variable>> = vec![];
        stack.push(table);

        assert!(State::parse_expression(&mut stack, *expr, &mut result).is_ok());
        assert_eq!(
            result,
            "ldc 2\nldc 3\nlod #0\nmul\nadd\nlod #1\nldc 2\ndiv\ngrt\n"
        );
    }

    #[test]
    fn valid_predicate_with_connectors() {
        let expr = vit_grammar::PredicateParser::new()
            .parse("2 + 3 * a > b / 2 and x == 2 or 2 != 2")
            .unwrap();
        println!("{expr:?}");
        let mut result = String::new();

        let mut table: HashMap<String, Variable> = HashMap::new();
        table.insert(
            "a".to_string(),
            Variable {
                address: 0,
                initialized: true,
            },
        );
        table.insert(
            "b".to_string(),
            Variable {
                address: 1,
                initialized: true,
            },
        );
        table.insert(
            "x".to_string(),
            Variable {
                address: 2,
                initialized: true,
            },
        );

        let mut stack: Vec<HashMap<String, Variable>> = vec![];
        stack.push(table);

        assert!(State::parse_expression(&mut stack, *expr, &mut result).is_ok());
        assert_eq!(
            result,
            "\
        ldc 2
ldc 3
lod #0
mul
add
lod #1
ldc 2
div
grt
lod #2
ldc 2
equ
and
ldc 2
ldc 2
neq
or\n"
        );
    }
}
