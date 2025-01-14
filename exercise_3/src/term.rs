use std::collections::HashMap;

// Representation of our terms
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(String),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
    Int(i64),
    Bool(bool),
    If(Box<Term>, Box<Term>, Box<Term>),
    PrimOp(PrimOp, Box<Term>, Box<Term>)
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum PrimOp {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Eq,  // ==
    Lt,  // <
    Gt,  // >
    // TODO: add `&&` and `||` operators
}

// Binds names to terms
pub type Env = HashMap<String, Term>;

// Represents a program with the main entry point and the environment
#[derive(Debug, Clone)]
pub struct Program { 
    pub env: Env,
    pub main: Term,
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

/// Helper function to create a literal term.
pub fn i(n: i64) -> Term {
    Term::Int(n)
}

/// Helper function to create a literal term.
pub fn b(b: bool) -> Term {
    Term::Bool(b)
}

/// Helper function to create an if term.
pub fn ifte(cond: Term, t1: Term, t2: Term) -> Term {
    Term::If(Box::new(cond), Box::new(t1), Box::new(t2))
}

/// Helper function to create a primitive operation term.
pub fn primop(op: PrimOp, t1: Term, t2: Term) -> Term {
    Term::PrimOp(op, Box::new(t1), Box::new(t2))
}

pub fn add(t1: Term, t2: Term) -> Term {
    primop(PrimOp::Add, t1, t2)
}

pub fn sub(t1: Term, t2: Term) -> Term {
    primop(PrimOp::Sub, t1, t2)
}

pub fn mul(t1: Term, t2: Term) -> Term {
    primop(PrimOp::Mul, t1, t2)
}   

pub fn div(t1: Term, t2: Term) -> Term {
    primop(PrimOp::Div, t1, t2)
}

pub fn eq(t1: Term, t2: Term) -> Term {
    primop(PrimOp::Eq, t1, t2)
}

pub fn lt(t1: Term, t2: Term) -> Term {
    primop(PrimOp::Lt, t1, t2)
}   

pub fn gt(t1: Term, t2: Term) -> Term {
    primop(PrimOp::Gt, t1, t2)
}
