use std::fmt::{Debug, Error, Formatter};

#[derive(Debug)]
pub enum ExprKind {
    Number(i32),
    Id(String),
    Error,
}

#[derive(Debug)]
pub struct Expr {
    pub node: ExprKind,
}

#[derive(Debug)]
pub struct Stmt {
    pub node: StmtKind,
}

#[derive(Debug)]
pub enum StmtKind {
    Local(Local),
    Expr(Expr),
}

#[derive(Debug)]
pub struct Local {
    pub id: String,
    // pub ty: Option<Ty>,
    pub init: Option<Expr>,
}

// impl Debug for Stmt {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         use self::Stmt::*;
//         match *self {
//             Bin {
//                 ref lhs,
//                 op,
//                 ref rhs,
//             } => write!(fmt, "{:?} {:?} {:?}", lhs, op, rhs),
//             Exp(ref exp) => write!(fmt, "{:?}", exp),
//             Assign { ref lhs, ref rhs } => write!(fmt, "{:?} {:?}", lhs, rhs),
//         }
//     }
// }

// impl Debug for Expr {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         use self::Expr::*;
//         match *self {
//             Number(n) => write!(fmt, "{:?}", n),
//             Id(ref n) => write!(fmt, "{:?}", n),
//             Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
//             Error => write!(fmt, "error"),
//         }
//     }
// }

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
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

// pub enum PRESERVE {
//     LET,
// }
