use crate::term::*;
use std::collections::HashSet;
use std::collections::HashMap;

/// Creates a new empty environment.
pub fn empty_env() -> Env {
    HashMap::new()
}

// Evaluates a term:
// env is a list of variable bindings
pub fn eval(env: &Env, term: &Term) -> Result<Term, String> {
    match term {
        Term::App(t1, t2) => {
            let l = eval(env, t1)?;
            let r = eval(env, t2)?;
            // If left is an abstraction, substitute the parameter with right
            if let Term::Abs(param, body) = l {
                // Remove the parameter from the environment
                // The parameter shadows any outer bindings
                let mut env = env.clone();
                env.remove(&param);

                eval(&env, &substitute(&body, &param, &r))
            } else {
                Ok(app(l, r))
            }
        }

        // eagerly evaluate the body of an abstraction
        Term::Abs(param, body) => Ok(abs(param, eval(env, body)?)), 

        Term::Var(x) => {
            // Look up the variable in the environment
            if let Some(t) = env.get(x) {
                Ok(t.clone())
            } else {
                Ok(var(x))
            }
        },


        Term::Int(n) => Ok(i(*n)),

        Term::Bool(v) => Ok(b(*v)),

        Term::If(cond, t1, t2) => {
            let c = eval(env, cond)?;
            if let Term::Bool(b) = c {
                if b {
                    return eval(env, t1);
                } else {
                    return eval(env, t2);
                }
            } else {
                Ok(ifte(c, *t1.clone(), *t2.clone()))
            }
        }

        Term::PrimOp(op, t1, t2) => {
            let l = eval(env, t1)?;
            let r = eval(env, t2)?;
            match op {
                PrimOp::Add | PrimOp::Sub | PrimOp::Mul | PrimOp::Div  => {
                    if let (Term::Int(n1), Term::Int(n2)) = (&l, &r) {
                        Ok(i(match op {
                            PrimOp::Add => n1 + n2,
                            PrimOp::Sub => n1 - n2,
                            PrimOp::Mul => n1 * n2,
                            PrimOp::Div => n1 / n2, // TODO: handle division by zero and add a test
                            _ => unreachable!()
                        }))
                    } else {
                        Ok(primop(*op, l, r))
                    }
                }

                PrimOp::Lt | PrimOp::Gt => {
                    if let (Term::Int(n1), Term::Int(n2)) = (&l, &r) {
                        Ok(b(match op {
                            PrimOp::Lt => n1 < n2,
                            PrimOp::Gt => n1 > n2,
                            _ => unreachable!()
                        }))
                    } else {
                        Ok(primop(*op, l, r))
                    }
                }
               
                PrimOp::Eq => {
                    if let (Term::Int(n1), Term::Int(n2)) = (&l, &r) {
                        Ok(b(n1 == n2))
                    } else if let (Term::Bool(b1), Term::Bool(b2)) = (&l, &r) {
                        Ok(b(b1 == b2))
                    } else {
                        Ok(primop(*op, l, r))
                    }
                }
            }
        }
    }
}


/// Replace all occurrences of a variable `var` in a `term` with `replacement`.
pub fn substitute(term: &Term, var: &str, replacement: &Term) -> Term {
    match term {
        Term::Var(x) if x == var => replacement.clone(),
        Term::Var(x) => Term::Var(x.clone()),
        Term::Abs(param, body) if param != var => {
            if free_variables(replacement).contains(param) {
                // Prevent variable capture by renaming the parameter
                let fresh_var = fresh_name(param, term, replacement);
                // Rename the parameter in the body
                let new_body = substitute(body, param, &Term::Var(fresh_var.clone()));
                abs(&fresh_var, substitute(&new_body, var, replacement))
            } else {
                abs(param, substitute(body, var, replacement))
            }
        }
        Term::Abs(param, body) => Term::Abs(param.clone(), body.clone()), // redeclaration of the same variable

        Term::App(t1, t2) => Term::App(
            Box::new(substitute(t1, var, replacement)),
            Box::new(substitute(t2, var, replacement)),
        ),
        Term::Int(n) => Term::Int(*n),
        Term::Bool(b) => Term::Bool(*b),
        Term::If(cond, t1, t2) => Term::If(
            Box::new(substitute(cond, var, replacement)),
            Box::new(substitute(t1, var, replacement)),
            Box::new(substitute(t2, var, replacement)),
        ),
        Term::PrimOp(op, t1, t2) => Term::PrimOp(
            *op,
            Box::new(substitute(t1, var, replacement)),
            Box::new(substitute(t2, var, replacement)),
        )
    }
}

/// Collects free variables in a term.
pub fn free_variables(term: &Term) -> HashSet<String> {
    match term {
        Term::Var(x) => {
            let mut set = HashSet::new();
            set.insert(x.clone());
            set
        }
        Term::Abs(param, body) => {
            let mut set = free_variables(body);
            set.remove(param);
            set
        }
        Term::App(t1, t2) => {
            let mut set = free_variables(t1);
            set.extend(free_variables(t2));
            set
        }
        Term::Int(_) | Term::Bool(_) => HashSet::new(),
        Term::If(cond, t1, t2) => {
            let mut set = free_variables(cond);
            set.extend(free_variables(t1));
            set.extend(free_variables(t2));
            set
        },
        Term::PrimOp(_, t1, t2) => {
            let mut set = free_variables(t1);
            set.extend(free_variables(t2));
            set
        }

    }
}

/// Generates a fresh variable name based on `base_name` that doesn't exist in `existing_vars`.
fn fresh_name(base_name: &str, term: &Term, replacement: &Term) -> String {
    let all_vars: HashSet<String> = collect_all_vars(term)
        .union(&collect_all_vars(replacement))
        .cloned()
        .collect();

    let mut fresh_var = base_name.to_string();
    let mut counter = 1;
    while all_vars.contains(&fresh_var) {
        fresh_var = format!("{}_{}", base_name, counter);
        counter += 1;
    }
    fresh_var
}

/// Collects all variables in a term (free and bound).
fn collect_all_vars(term: &Term) -> HashSet<String> {
    match term {
        Term::Var(x) => {
            let mut vars = HashSet::new();
            vars.insert(x.clone());
            vars
        }
        Term::Abs(param, body) => {
            let mut vars = collect_all_vars(body);
            vars.insert(param.clone());
            vars
        }
        Term::App(t1, t2) => {
            let mut vars = collect_all_vars(t1);
            vars.extend(collect_all_vars(t2));
            vars
        }
        Term::Int(_) | Term::Bool(_) => HashSet::new(),
        Term::If(cond, t1, t2) => {
            let mut vars = collect_all_vars(cond);
            vars.extend(collect_all_vars(t1));
            vars.extend(collect_all_vars(t2));
            vars
        }
        Term::PrimOp(_, t1, t2) => {
            let mut vars = collect_all_vars(t1);
            vars.extend(collect_all_vars(t2));
            vars
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_variables() {
        let term = abs("x", app(var("x"), var("y")));
        let free_vars = free_variables(&term);
        let expected_vars: HashSet<_> = vec!["y".to_string()].into_iter().collect();
        assert_eq!(free_vars, expected_vars);
    }

    #[test]
    fn test_substitute_simple() {
        let term = app(var("x"), var("y"));
        let replacement = var("z");
        let substituted = substitute(&term, "x", &replacement);
        let expected = app(var("z"), var("y"));
        assert_eq!(substituted, expected);
    }

    #[test]
    fn test_substitute_capture_avoiding() {
        let term = abs("x", var("y"));
        let replacement = var("x");
        let substituted = substitute(&term, "y", &replacement);
        let expected = abs("x_1", var("x"));
        assert_eq!(substituted, expected);
    }

    #[test]
    fn test_eval_simple_application() {
        // (λx. x) y -> y
        let term = app(abs("x", var("x")), var("y"));
        let evaluated = eval(&empty_env(), &term).unwrap();
        let expected = var("y");
        assert_eq!(evaluated, expected);
    }

    #[test]
    fn test_eval_nested_abstraction() {
        // (λx. λy. x) z -> λy. z
        let term = app(abs("x", abs("y", var("x"))), var("z"));
        let evaluated = eval(&empty_env(), &term).unwrap();
        let expected = abs("y", var("z"));
        assert_eq!(evaluated, expected);
    }

    #[test]
    fn test_free_variables_in_nested_abstraction() {
        let term = abs("x", abs("y", app(var("x"), var("z"))));
        let free_vars = free_variables(&term);
        let expected_vars: HashSet<_> = vec!["z".to_string()].into_iter().collect();
        assert_eq!(free_vars, expected_vars);
    }

    #[test]
    fn test_substitute_in_nested_abstraction() {
        let term = abs("x", app(var("x"), var("y")));
        let replacement = var("z");
        let substituted = substitute(&term, "y", &replacement);
        let expected = abs("x", app(var("x"), var("z")));
        assert_eq!(substituted, expected);
    }

    #[test]
    fn test_eval_complex_application() {
        let term = app(abs("x", app(abs("y", var("y")), var("x"))), var("z"));
        let evaluated = eval(&empty_env(), &term).unwrap();
        let expected = var("z");
        assert_eq!(evaluated, expected);
    }
}
