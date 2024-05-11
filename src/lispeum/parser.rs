use super::expression::Expr;

pub fn parse(tokens: &[String]) -> Result<(Expr, &[String]), String> {
    let (first_token, rest) = tokens.split_first()
        .ok_or_else(|| "Unexpected end of input".to_string())?;

    match first_token.as_str() {
        "(" => {

            let (next_token, next_rest) = rest.split_first().ok_or_else(|| "Expected token after '('")?;

            // Check if we are parsing a function definition
            if next_token == "fn" {
                // Assume next items are the parameter list and the function body
                let (params_expr, after_params) = parse(next_rest)?;
                if let Expr::List(params) = params_expr {
                    // Ensure the params are all symbols
                    let params = params.into_iter().map(|expr| {
                        if let Expr::Symbol(sym) = expr {
                            Ok(sym)
                        } else {
                            Err("Function parameters must be symbols".to_string())
                        }
                    }).collect::<Result<Vec<_>, _>>()?;

                    let (body, remaining_tokens) = parse(after_params)?;
                    // Make sure to skip the closing ')' of the function definition
                    let remaining_tokens = remaining_tokens.get(1..).ok_or_else(|| "Expected closing ')' after function body".to_string())?;
                    return Ok((Expr::Function(params, Box::new(body)), remaining_tokens));
                } else {
                    return Err("Expected parameter list after 'fn'".to_string());
                }
            }
            
            let mut list_items = Vec::new();
            let mut inner_tokens = rest;
            while inner_tokens.get(0).map(String::as_str) != Some(")") {
                let (next_expr, remaining_tokens) = parse(inner_tokens)?;
                list_items.push(next_expr);
                inner_tokens = remaining_tokens;
            }
            // Skip the closing ")"
            Ok((Expr::List(list_items), &inner_tokens[1..]))
        },
        ")" => Err("Unexpected closing parenthesis".to_string()),
        token if token.starts_with("\"") && token.ends_with("\"") => {
            Ok((Expr::String(token[1..token.len()-1].to_string()), rest))
        },
        token => {
            match token.parse::<i64>() {
                Ok(number) => Ok((Expr::Integer(number), rest)),
                Err(_) => Ok((Expr::Symbol(token.to_string()), rest)),
            }
        },
    }
}
