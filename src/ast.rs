use std::fmt::{Debug, Error, Formatter};

pub enum Expr {
    Number(i32),
    Id(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Error,
}

pub enum Stmt {
    Bin {
        lhs: Box<Expr>,
        op: Opcode,
        rhs: Box<Expr>,
    },
    Exp(Box<Expr>),
    Assign {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

impl Debug for Stmt {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Stmt::*;
        match *self {
            Bin {
                ref lhs,
                op,
                ref rhs,
            } => write!(fmt, "{:?} {:?} {:?}", lhs, op, rhs),
            Exp(ref exp) => write!(fmt, "{:?}", exp),
            Assign { ref lhs, ref rhs } => write!(fmt, "{:?} {:?}", lhs, rhs),
        }
    }
}

pub enum PRESERVE {
    LET,
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Id(ref n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Error => write!(fmt, "error"),
        }
    }
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}
