use super::{environment::Environment, expression::Expr, special::SpecialFunctions};

pub fn evaluate(expr: Expr, env: &mut Environment, special_funcs: &SpecialFunctions) -> Result<Expr, String> {
    match expr {
        Expr::List(exprs) => {
            if let Some(Expr::Symbol(s)) = exprs.get(0) {
                // First, check for user-defined functions in the environment
                let user_def_fn: Option<Expr> = match env.get(s) {
                    Some(res) => Some(res.clone()),
                    None => None,
                };

                if let Some(Expr::Function(fn_args, fn_body)) = user_def_fn {
                    // Ensure correct number of arguments
                    if fn_args.len() != exprs.len() - 1 {
                        return Err(format!("expected {} arguments, got {}", fn_args.len(), exprs.len() - 1));
                    }
                    // Prepare to set parameters with evaluated arguments
                    for (param, arg) in fn_args.iter().zip(exprs.iter().skip(1)) {
                        let evaluated_arg = evaluate(arg.clone(), env, special_funcs)?;
                        env.set(param.clone(), evaluated_arg);
                    }
                    // Evaluate the function body with updated environment
                    evaluate(*fn_body.clone(), env, special_funcs)
                } else {
                    // If not a user-defined function, check for special functions
                    if special_funcs.functions.contains_key(s) {
                        special_funcs.call(s, exprs[1..].to_vec(), env)
                    } else {
                        // If not a special function either, return the list as a normal expression
                        Ok(Expr::List(exprs))
                    }
                }
            } else {
                // If the first element is not a Symbol, evaluate all expressions in the list normally
                let mut evaluated_exprs = Vec::new();
                for expr in exprs {
                    evaluated_exprs.push(evaluate(expr.clone(), env, special_funcs)?);
                }
                Ok(Expr::List(evaluated_exprs))
            }
        },
        Expr::Symbol(s) => {
            match env.get(&s) {
                Some(value) => Ok(value.clone()),
                None => Ok(Expr::Symbol(s))
            }
        },
        _ => Ok(expr),
    }
}
