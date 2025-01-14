use crate::term::*;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, digit1, multispace0, satisfy},
    combinator::{map, map_res, recognize},
    error::{make_error, ErrorKind},
    multi::many1,
    sequence::{delimited, pair, separated_pair, terminated},
    IResult,
};
use std::{collections::HashMap, str::FromStr};

// Helper function to parse whitespace
fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

fn parse_var(input: &str) -> IResult<&str, Term> {
    map(parse_identifier, |s: String| Term::Var(s.to_string()))(input)
}

fn parse_int(input: &str) -> IResult<&str, Term> {
    map_res(digit1, |s: &str| i64::from_str(s).map(Term::Int))(input)
}

fn parse_bool(input: &str) -> IResult<&str, Term> {
    alt((
        map(tag("true"), |_| Term::Bool(true)),
        map(tag("false"), |_| Term::Bool(false)),
    ))(input)
}

// Syntax: if x then y else z
fn parse_if_then_else(input: &str) -> IResult<&str, Term> {
    let (input, _) = ws(tag("if"))(input)?;
    let (input, cond) = ws(parse_expression)(input)?;
    let (input, _) = ws(tag("then"))(input)?;
    let (input, t1) = ws(parse_expression)(input)?;
    let (input, _) = ws(tag("else"))(input)?;
    let (input, t2) = ws(parse_expression)(input)?;
    Ok((input, ifte(cond, t1, t2)))
}

// Syntax: x + y
fn parse_binary_op(input: &str) -> IResult<&str, Term> {
    let (input, left) = ws(parse_expression)(input)?; // Parse the left operand
    let (input, op) = ws(parse_prim_op)(input)?; // Parse the operator with whitespace handling
    let (input, right) = ws(parse_expression)(input)?; // Parse the right operand
    Ok((input, primop(op, left, right)))
}

fn parse_prim_op(input: &str) -> IResult<&str, PrimOp> {
    alt((
        map(tag("+"), |_| PrimOp::Add),
        map(tag("-"), |_| PrimOp::Sub),
        map(tag("*"), |_| PrimOp::Mul),
        map(tag("/"), |_| PrimOp::Div),
        map(tag("=="), |_| PrimOp::Eq),
        map(tag("<"), |_| PrimOp::Lt),
        map(tag(">"), |_| PrimOp::Gt),
    ))(input)
}

fn parse_identifier(input: &str) -> IResult<&str, String> {
    let keywords = ["if", "then", "else", "true", "false"];
    let (input, name) = recognize(pair(
        satisfy(|c: char| c.is_ascii_alphabetic()),
        take_while(|c: char| c.is_ascii_alphanumeric() || c == '_'),
    ))(input)?;

    if keywords.contains(&name) {
        Err(nom::Err::Error(make_error(input, ErrorKind::Satisfy)))
    } else {
        Ok((input, name.to_string()))
    }
}

// Syntax: λx. x
fn parse_abs(input: &str) -> IResult<&str, Term> {
    // TODO: Also accept Haskell style notation: \x -> x
    let (input, _) = ws(char('λ'))(input)?; 
    let (input, var) = ws(parse_identifier)(input)?;
    let (input, _) = ws(char('.'))(input)?; 
    let (input, body) = ws(parse_expression)(input)?;
    Ok((input, abs(&var, body)))
}

// Syntax: x y
fn parse_app(input: &str) -> IResult<&str, Term> {
    let (input, l) = ws(parse_expression)(input)?;
    let (input, r) = ws(parse_expression)(input)?;
    Ok((input, app(l, r)))
}

fn parse_complex_expression(input: &str) -> IResult<&str, Term> {
    alt((parse_abs, parse_app, parse_if_then_else, parse_binary_op))(input)
}

// Syntax: Parenteses are used around complex expressions
pub fn parse_expression(input: &str) -> IResult<&str, Term> {
    alt((
        parse_var,
        parse_int,
        parse_bool,
        delimited(ws(char('(')), parse_complex_expression, ws(char(')'))),
    ))(input)
}

// Syntax: x = y; z = w;
pub fn parse_program(input: &str) -> IResult<&str, Vec<(String, Term)>> {
    many1(terminated(
        separated_pair(ws(parse_identifier), ws(tag("=")), ws(parse_expression)),
        ws(tag(";")),
    ))(input)
}

// Parse the main program. Expect the main expression to be defined.
pub fn parse_main_program(input: &str) -> Result<Program, &str> {
    let (input, prog) = parse_program(input).expect("parse error");
    assert!(input.is_empty(), "parse error: trailing input");

    let mut env = HashMap::new();
    for (name, term) in prog {
        env.insert(name, term);
    }

    let main = env.get("main").expect("main function not found");
    
    Ok(Program {
        env: env.clone(),
        main: main.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    type R = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_abs() -> R {
        let input = "(λx. (x y))";
        let (_, term) = parse_expression(input)?;
        assert_eq!(abs("x", app(var("x"), var("y"))), term);
        Ok(())
    }

    #[test]
    fn test_if() -> R {
        let input = "(λx. (if x then y  else 0))";
        let (_, term) = parse_expression(input)?;
        assert_eq!(abs("x", ifte(var("x"), var("y"), i(0))), term,);
        Ok(())
    }

    #[test]
    fn test_app_app() -> R {
        let input = "((x y) z)";
        let (_, term) = parse_expression(input)?;
        assert_eq!(app(app(var("x"), var("y")), var("z")), term,);
        Ok(())
    }

    // TODO: Add some more tests to cover the remaining syntax elements
}
