use lc::term::*;
use lc::eval::*;

/// Driver code to run the lambda calculus evaluator.
fn main() {
    // (λx. x y) z
    let input = app(abs("x", app(var("x"), var("y"))), var("z"));

    println!("Original term: {}", input);
    let result = eval(&input);
    println!("Evaluated term: {}", result);
}