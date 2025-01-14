use crate::term::*;
use std::fmt;

/// Pretty prints a term.
pub fn pretty_print(term: &Term) -> String {
    match term{
        Term::Var(s)=>
            s.clone(),
        Term::Abs(s, t) =>
            format!("λ{}. {}", s, pretty_print(&t)),
        Term::App(t1, t2) => 
            format!("({} {})", pretty_print(&t1), pretty_print(&t2)),
    }
}

/// Display trait implementation for Term.
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", pretty_print(self))
    }
}