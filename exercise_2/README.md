# Assignment 8: Slices, Strings and Lambda Calculus

In this assignment you will learn about slices, common collections and lambda calculus in Rust.

## 1. Warm-up Exercise

Solve the Rustlings exercises related to ownership (`06_move_semantics/`). 

## 2. Slices 
Read the [Slices chapter](https://doc.rust-lang.org/book/ch04-03-slices.html) from the Rust book to learn about slices in Rust. Slices are references to a contiguous sequence of elements in a collection.

## 3. Common Collections
Read the [Common Collections chapter](https://doc.rust-lang.org/stable/book/ch08-00-common-collections.html) from the Rust book to learn about `Vec`, `String` and `HashMap` in Rust.
Then solve the related exercises (`05_vecs/`, `09_strings/`, `11_hashmaps`) in Rustlings.

## 4. Lambda Calculus Interpreter
In this part you will implement a simple [Lambda Calculus](https://en.wikipedia.org/wiki/Lambda_calculus) interpreter in Rust. Lambda calculus is a formal system in mathematical logic for expressing computation based on function abstraction and application. It was introduced by Alonzo Church in the 1930s and is at the core of functional programming languages like Haskell and Lisp.

The lambda calculus consists of three basic constructs:
- Variables: `x`, `y`, `z`, ...
- Abstraction: `λx. M`, where `x` is a variable and `M` is a lambda term.
- Application: `M N`, where `M` and `N` are lambda terms.

A lambda term is either a variable, an abstraction or an application. The following are examples of lambda terms:
- `x`
- `λx. x`
- `λx. (λy. x y)`
- `(λx. x) (λy. y)`
- `λf. (λx. f (f (f x)))`

The lambda calculus is Turing complete, which means that it can express any computation that can be performed by a computer. 
In this exercise you will implement a simple interpreter for the lambda calculus that can evaluate lambda terms.

### Task
Implement a simple lambda calculus interpreter in Rust. The interpreter should be able to evaluate lambda terms and reduce them to their normal form. The lambda terms should be represented as a Rust data structure. You can use the following data type to represent lambda terms:

```rust
enum Term {
    Var(String),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
}
```

The `Var` variant represents a variable, the `Abs` variant represents an abstraction (an anonymous function) and the `App` variant represents function application. The `String` type is used to represent variable names.

You can use the following example to represent the lambda term `λx. x`:
```rust
let term = Term::Abs("x".to_string(), Box::new(Term::Var("x".to_string())));
```

We provide a small embedded domain specific language (EDSL) for creating lambda terms. You can use the `var`, `abs` and `app` functions to create lambda terms. The term from the previous example can be created as follows:
```rust
let term = abs("x", var("x"));
```

#### 1. Pretty Printing
Implement a pretty printer for the lambda terms. The pretty printer should implement the following function:

```rust
fn pretty_print(term: &Term) -> String {
    // Pretty print the lambda term
}
```

It's up to you whether you want to omit superfluous parentheses or not. The pretty printer should be able to print lambda terms in a human-readable format.
Both variants are fine:
- `λx. λy. x`
- `λx. (λy. x)`

Add some tests to test your pretty printer with different lambda terms.

#### 2. Evaluation
Implement an interpreter for the lambda calculus that can evaluate lambda terms. The interpreter should implement the following function:

```rust
fn eval(term: &Term) -> Term {
    // Evaluate the lambda term and return the normal form
}
```

The `eval` function should evaluate the lambda term and reduce it. You can use the following rules to evaluate lambda terms:
- **Variable**: A variable evaluates to itself.
- **Abstraction**: An abstraction evaluates to itself.
- **Application**: An application evaluates by substituting the argument into the body of the abstraction.

For example, the lambda term `(λy. y)(λx. x)` should evaluate to `λx. x`.

#### Capture-avoiding substitution
When performing substitution in the lambda calculus, it is important to avoid variable capture. 
Example:
```
(λx. (λy. x y)) y
```
A naive substitution would yield `(λy. y y)`, which is incorrect because the variable `y` is captured by the inner lambda term. The solution is to rename the bound variable in the abstraction to avoid capture (rename `y` to `y_1`):
```
(λx. (λy_1. x y_1)) y
```
This yields `(λy_1. y y_1)`, which is the correct result.

A function to perform capture-avoiding substitution is provided. Here is its signature:
```rust
fn subst(term: &Term, var: &str, replacement: &Term) -> Term {
    // Perform capture avoiding substitution
}
```

You can test your interpreter by evaluating lambda terms and comparing the result with the expected normal form. To get some inspiration for examples, you can look at the [Church encoding](https://en.wikipedia.org/wiki/Church_encoding) of natural numbers, booleans and pairs in the lambda calculus.

Add more test cases to test your interpreter with different lambda terms.
