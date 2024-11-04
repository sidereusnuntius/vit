use std::fmt;

pub type Identifier = String;

#[derive(Debug)]
pub enum Statement {
    Declaration(Identifier, Option<Box<Expr>>),
    Assignment(Identifier, Box<Expr>),
    Read(Identifier),
    Predicate(Box<Expr>),
}

pub enum Expr {
    Number(i32),
    Id(Identifier),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Predicate(Box<Expr>, Opcode, Box<Expr>),
}

pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    And,
    Or,
    Not,
    Eq,
    Neq,
    Grt,
    Let,
    Geq,
    Leq,
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Opcode::Add => "+",
            Opcode::Sub => "-",
            Opcode::Mul => "*",
            Opcode::Div => "/",
            Opcode::Exp => "^",
            Opcode::And => "and",
            Opcode::Or => "or",
            Opcode::Not => "!",
            Opcode::Eq => "==",
            Opcode::Neq => "!=",
            Opcode::Grt => ">",
            Opcode::Let => "<",
            Opcode::Geq => ">=",
            Opcode::Leq => "<=",
        })
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Expr::Number(n) => format!("{n}"),
            Expr::Op(l, op, r) => format!("({:?} {op:?} {:?})",
                *l, *r),
            Expr::Predicate(l, op, r) => format!("({:?} {op:?} {:?})",
                *l, *r),
            Expr::Id(ID) => ID.clone(),
        })
    }
}
