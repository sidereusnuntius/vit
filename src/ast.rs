use core::fmt;

pub type Identifier = String;

#[derive(Debug)]
pub enum Statement {
    Declaration(Identifier, Option<Box<Expr>>),
    Assignment(Identifier, Box<Expr>),
    Read(Identifier),
    If(Box<Expr>, Vec<Statement>, Option<Vec<Statement>>),
    Until(Box<Expr>, Vec<Statement>),
    WriteLiteral(String),
    WriteId(Identifier),
    Loop(Vec<Statement>),
    Break,
}

pub enum Expr {
    Number(bool, Box<Expr>),
    Integer(i32),
    Float(f32),
    Id(Identifier),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Predicate(Box<Expr>, Opcode, Box<Expr>),
}

#[derive(PartialEq)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Mod,
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

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Opcode::Add => "+",
            Opcode::Sub => "-",
            Opcode::Mul => "*",
            Opcode::Div => "/",
            Opcode::Mod => "%",
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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Expr::Number(sign, num) => format!("{}{}", if *sign { "-" } else { "" }, num),
            Expr::Integer(n) => format!("{n}"),
            Expr::Float(n) => format!("{n}"),
            Expr::Op(l, op, r) => format!("({} {} {})",
                *l, op, *r),
            Expr::Predicate(l, op, r) => format!("({} {} {})",
                *l, op, *r),
            Expr::Id(id) => id.clone(),
        })
    }
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Opcode::Add => "+",
            Opcode::Sub => "-",
            Opcode::Mul => "*",
            Opcode::Div => "/",
            Opcode::Mod => "%",
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
            Expr::Number(sign, num) => format!("{}{}", if *sign { "-" } else { "" }, num),
            Expr::Integer(n) => format!("{n}"),
            Expr::Float(n) => format!("{n}"),
            Expr::Op(l, op, r) => format!("({} {} {})",
                *l, op, *r),
            Expr::Predicate(l, op, r) => format!("({} {} {})",
                *l, op, *r),
            Expr::Id(id) => id.clone(),
        })
    }
}
