// adder.rs

use crate::lispeum::{environment::Environment, evaluator::evaluate, expression::Expr};

use super::SpecialFunctions;

pub fn add(args: Vec<Expr>, env: &mut Environment, special_funcs: &SpecialFunctions) -> Result<Expr, String> {
    let sum = args.into_iter().try_fold(0, |acc, arg| {
        let evaluated_arg = evaluate(arg, env, special_funcs)?;
        match evaluated_arg {
            Expr::Integer(n) => Ok(acc + n),
            _ => Err("All arguments to '+' must evaluate to integers".to_string()),
        }
    })?;

    Ok(Expr::Integer(sum))
}
