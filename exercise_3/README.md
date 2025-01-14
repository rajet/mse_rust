# Assignment 9: Error Handling, Functional Language Features and Extending the Lambda Calculus Interpreter

In this assignment you will learn about error handling in Rust and extend your lambda calculus interpreter to a small pure functional programming language.

## 1. Error Handling 
Read the [Error Handling chapter](https://doc.rust-lang.org/stable/book/ch09-00-error-handling.html) from the Rust book. Then complete the related exercises (`13_error_handling/`) in Rustlings.


## 2. Functional Language Features
Read sections [13.1](https://doc.rust-lang.org/stable/book/ch13-01-closures.html) and [13.2](https://doc.rust-lang.org/stable/book/ch13-02-iterators.html) of the Rust book. 


## 3. Extending the Lambda Calculus Interpreter
In the previous assignment you implemented a simple lambda calculus interpreter. While lambda calculus is Turing complete, it is not very practical for writing programs.

For example we had to encode numbers as functions:
```rust
// Encoding of the value two: (λf.λx.f (f x))
let two = abs("f", abs("x", app(var("f"), app(var("f"), var("x")))));

// Encoding of the value three: (λf.λx.f (f (f x)))
let three = abs("f", abs("x", app(var("f"), app(var("f"), app(var("f"), var("x"))))));

// Addition for church encoded numbers: (λm.λn.λf.λx.m f (n f x))
let church_add = abs("m", abs("n", abs("f", abs("x", app(app(var("m"), var("f")), app(app(var("n"), var("f")), var("x")))))));

// Putting it together:
let sum = app(app(church_add, two), three);
// Evaluate the expression:
let result = eval(&empty_env(), &sum).unwrap();

// The result is the church encoding of the number five: (λf.λx.f (f (f (f (f x)))))
assert_eq!(result, abs("f", abs("x", app(var("f"), app(var("f"), app(var("f"), app(var("f"), app(var("f"), var("x")))))))));
```

As you can see, this is not very practical 🤯.

## Extending the Interpreter
In this assignment we will extend the lambda calculus interpreter to a small pure functional programming language. 
We will add the following features:
- Integers (`1`, `42`)
- Booleans (`true`, `false`)
- Basic arithmetic and comparison operators (`+`, `-`, `*`, `/`, `==`, `<`, `>`)
- If-Then-Else expressions (`if <condition> then <then-branch> else <else-branch>`)
- Top-level bindings:
```
x = 5
fact = (λn. if (n == 0) then 1 else (n * (fact (n - 1))))
main = fact x
```

To support these features we added the following value constructors to the `Term` enum:
```rust
pub enum Term {
    Var(String),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
    // Extensions
    Int(i64),
    Bool(bool),
    If(Box<Term>, Box<Term>, Box<Term>),
    PrimOp(PrimOp, Box<Term>, Box<Term>)
}


pub enum PrimOp {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Eq,  // ==
    Lt,  // <
    Gt,  // >
}
```


## Adding a Parser
We also added a parser for the language. The parser is implemented using the [nom](https://github.com/rust-bakery/nom) parser combinator library. The parser is already implemented and you can find it in the `src/parser.rs` file. The syntax is kept simple; every composite expression is wrapped in parentheses.


## Tasks
- Find the `todo!` / `TODO:` in the given code and implement the missing parts.
- \[Optional\] Make the language your own!
  - Add more operators (e.g. `not`, `mod`, `pow`, ...)
  - Add support for tuples, lists, records, ...
  - Change the syntax to your liking.
