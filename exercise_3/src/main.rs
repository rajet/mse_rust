use fun::term::*;
use fun::eval::*;
use fun::parser::*;

/// Driver code to run the lambda calculus evaluator.
fn main() -> Result<(), String> {
    // Task: Add the numbers 2 and 3

    // Before extending the interpreter:
    // Encoding of the value two: (λf.λx.f (f x))
    let two = abs("f", abs("x", app(var("f"), app(var("f"), var("x")))));
    
    // Encoding of the value three: (λf.λx.f (f (f x)))
    let three = abs("f", abs("x", app(var("f"), app(var("f"), app(var("f"), var("x"))))));
 
    // Addition for church encoded numbers: (λm.λn.λf.λx.m f (n f x))
    let church_add = abs("m", abs("n", abs("f", abs("x", app(app(var("m"), var("f")), app(app(var("n"), var("f")), var("x")))))));
 
    // Add two and three
    let sum = app(app(church_add, two), three);
 
    let result = eval(&empty_env(), &sum)?;
    // Prints the encoding of 5: λf. λx. f (f (f (f (f x))))
    println!("The result of adding 2 and 3 is: {}\n", result);


    // After extending the interpreter:
    let sum = add(i(2), i(3));
    println!("Direct representation of 2 + 3: {:?}", sum);
    let result = eval(&empty_env(), &sum)?;
    println!("The result of adding 2 and 3 is: {}\n", result);  // Should print 5
 

    // After adding a parser:
    let (_, expr) = parse_expression("(2 + 3)").expect("parse error");
    println!("Parsed expression: {:?}", expr);
    let result = eval(&empty_env(), &expr)?;
    println!("The result of adding 2 and 3 is: {}", result);  // Should print 5
    Ok(())
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        // Task: Factorial of 5
        let input = r#"
            fac = (λn. (if (n == 0) then 1 else (n * (fac (n - 1)))));
            main = (fac 5);
        "#;

        let prog = parse_main_program(input).unwrap();
        let result = eval(&prog.env, &prog.main).unwrap();
        assert_eq!(i(120), result);
    }

    #[test]
    fn test_fibonacci() {
        // TODO: implement the fibonacci function https://en.wikipedia.org/wiki/Fibonacci_sequence
        let input = r#"
            fib = TODO: Implement fibonacci function;
            main = (fib 10);
        "#;

        let prog = parse_main_program(input).unwrap();
        let result = eval(&prog.env, &prog.main).unwrap();
        assert_eq!(i(55), result);
    }
}
