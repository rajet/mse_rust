#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(String),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
}

/// Helper function to create a variable term.
pub fn var(name: &str) -> Term {
    Term::Var(name.to_string())
}

/// Helper function to create an abstraction term.
pub fn abs(param: &str, body: Term) -> Term {
    Term::Abs(param.to_string(), Box::new(body))
}

/// Helper function to create an application term.
pub fn app(t1: Term, t2: Term) -> Term {
    Term::App(Box::new(t1), Box::new(t2))
}
