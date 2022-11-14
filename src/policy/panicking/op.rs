use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Op {
    Abs,
    Add,
    Div,
    DivEuclid,
    Mul,
    Neg,
    Pow,
    Rem,
    RemEuclid,
    Shl,
    Shr,
    Sub,
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Op::*;
        write!(f, "{}", match self {
            Abs => "Abs",
            Add => "Add",
            Div => "Div",
            DivEuclid => "DivEuclid",
            Mul => "Mul",
            Neg => "Neg",
            Pow => "Pow",
            Rem => "Rem",
            RemEuclid => "RemEuclid",
            Shl => "Shl",
            Shr => "Shr",
            Sub => "Sub",
        })
    }
}
