// adder.rs

use crate::lispeum::{environment::Environment, expression::Expr};

use super::SpecialFunctions;

pub fn add(args: Vec<Expr>, _env: &mut Environment, _special_funcs: &SpecialFunctions) -> Result<Expr, String> {
    let sum = args.into_iter().try_fold(0, |acc, arg| {
        match arg {
            Expr::Integer(n) => Ok(acc + n),
            _ => Err("All arguments to '+' must be integers".to_string()),
        }
    })?;

    Ok(Expr::Integer(sum))
}
