use super::expression::Expr;

pub fn parse(tokens: &[String]) -> Result<(Expr, &[String]), String> {
    let (first_token, rest) = tokens.split_first()
        .ok_or_else(|| "Unexpected end of input".to_string())?;

    match first_token.as_str() {
        "(" => {
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
