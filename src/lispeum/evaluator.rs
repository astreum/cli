use super::{environment::Environment, expression::Expr, special::SpecialFunctions};

pub fn evaluate(expr: Expr, env: &mut Environment, special_funcs: &SpecialFunctions) -> Result<Expr, String> {
    match expr {
        Expr::List(exprs) => evaluate_list(exprs, env, special_funcs),
        Expr::Symbol(s) => {
            // Attempt to find the symbol in the environment
            match env.get(&s) {
                Some(value) => Ok(value.clone()),
                None => Err(format!("Symbol '{}' not found", s)),
            }
        },
        // Handle other cases like integers, strings, etc., directly
        _ => Ok(expr),
    }
}

pub fn evaluate_list(exprs: Vec<Expr>, env: &mut Environment, special_funcs: &SpecialFunctions) -> Result<Expr, String> {
    if exprs.is_empty() {
        return Err("Cannot evaluate an empty list".to_string());
    }

    match &exprs[0] {
        Expr::Symbol(s) => {
            // Delegate to `SpecialFunctions` if it's a special function
            if special_funcs.functions.contains_key(s) {
                return special_funcs.call(s, exprs[1..].to_vec(), env);
            } else {
                // Handle regular function calls or look up lists by name
                // This could involve finding a function in `env` and applying it to the arguments
                // Or simply evaluating each expression in the list if it's not a function call
                return Err(format!("Handling of regular functions or named lists not implemented: '{}'", s));
            }
        },
        _ => {
            // If the first item is not a symbol, evaluate each item in the list
            // This branch might need to loop through each expression in `exprs`, evaluate them, and construct a new list
            let mut evaluated_exprs = Vec::new();
            for expr in exprs {
                evaluated_exprs.push(evaluate(expr.clone(), env, special_funcs)?);
            }
            Ok(Expr::List(evaluated_exprs))
        },
    }
}
