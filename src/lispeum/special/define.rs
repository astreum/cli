// define.rs

use crate::lispeum::{environment::Environment, evaluator::evaluate, expression::Expr};

/// Defines a new variable or updates an existing one in the given environment.
pub fn define(args: Vec<Expr>, env: &mut Environment) -> Result<Expr, String> {
    if args.len() != 2 {
        return Err("define expects exactly two arguments".to_string());
    }

    let name = match &args[0] {
        Expr::Symbol(s) => s.clone(),
        _ => return Err("First argument to define must be a symbol".to_string()),
    };

    let value = match evaluate(args[1].clone(), env, ) {
        Ok(val) => val,
        Err(e) => return Err(e),
    };

    env.set(name, value);

    Ok(Expr::Symbol(name))  // Returning the name of the defined variable might vary based on your language's design
}
