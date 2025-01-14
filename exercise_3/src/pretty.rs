use crate::term::*;
use std::fmt;

// Pretty prints a term.
pub fn pretty_print(term: &Term) -> String {
    match term {
        Term::Var(name) => name.clone(),
        Term::Abs(param, body) => format!("(λ{}. {})", param, pretty_print(body)),
        Term::App(t1, t2) => format!("({} {})", pretty_print(t1), pretty_print(t2)),
        Term::Int(i) => format!("{i}"),
        Term::Bool(b) => format!("{b}"),
        Term::If(cond, t1, t2) => format!(
            "(if {} then {} else {})",
            pretty_print(cond),
            pretty_print(t1),
            pretty_print(t2)
        ),
        Term::PrimOp(op, t1, t2) => format!(
            "({} {} {})",
            pretty_print(t1),
            match op {
                PrimOp::Add => "+",
                PrimOp::Sub => "-",
                PrimOp::Mul => "*",
                PrimOp::Div => "/",
                PrimOp::Eq => "==",
                PrimOp::Lt => "<",
                PrimOp::Gt => ">",
            },
            pretty_print(t2)
        ),
    }
}

/// Display trait implementation for Term.
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", pretty_print(self))
    }
}
