use super::{environment::Environment, expression::Expr, special::SpecialFunctions};

pub fn evaluate(expr: Expr, env: &mut Environment, special_funcs: &SpecialFunctions) -> Result<Expr, String> {
    match expr {
        Expr::List(exprs) => evaluate_list(exprs, env, special_funcs),
        Expr::Symbol(s) => {
            match env.get(&s) {
                Some(value) => Ok(value.clone()),
                None => Err(format!("Symbol '{}' not found", s)),
            }
        },
        _ => Ok(expr),
    }
}

pub fn evaluate_list(exprs: Vec<Expr>, env: &mut Environment, special_funcs: &SpecialFunctions) -> Result<Expr, String> {
    if exprs.is_empty() {
        return  Ok(Expr::List(vec![]));
    }

    match &exprs[0] {
        Expr::Symbol(s) => {
            if special_funcs.functions.contains_key(s) {
                return special_funcs.call(s, exprs[1..].to_vec(), env);
            } else {
                // try and get the object in env
                // if its a function, call the function and arguments 
                return Ok(Expr::List(exprs));
            }
        },
        _ => {
            let mut evaluated_exprs = Vec::new();
            for expr in exprs {
                evaluated_exprs.push(evaluate(expr.clone(), env, special_funcs)?);
            }
            Ok(Expr::List(evaluated_exprs))
        },
    }
}
