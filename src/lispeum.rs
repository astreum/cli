use std::{collections::HashMap, fmt};

#[derive(Clone, Debug)]
pub enum LispExpr {
    List(Vec<LispExpr>),
    Symbol(String),
}

impl fmt::Display for LispExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LispExpr::List(elements) => {
                let inner: Vec<String> = elements.iter().map(|e| format!("{}", e)).collect();
                write!(f, "({})", inner.join(" "))
            },
            LispExpr::Symbol(s) => write!(f, "{}", s),
        }
    }
}

pub struct Environment {
    data: HashMap<String, LispExpr>,
}

impl Environment {
    pub fn new() -> Self {
        let env = Self { data: HashMap::new() };
        env
    }

    fn set(&mut self, name: String, value: LispExpr) {
        self.data.insert(name, value);
    }

    fn get(&self, name: &str) -> Option<&LispExpr> {
        self.data.get(name)
    }
}

pub fn evaluate(expr: LispExpr, env: &mut Environment) -> Result<LispExpr, String> {
    match expr {
        LispExpr::List(ref expressions) => {

            match expressions.get(0) {
                Some(first_expression) => {

                    match first_expression {
                        LispExpr::Symbol(first_symbol) => {

                            match &first_symbol[..] {
                                "def" => {
                                    
                                    if let LispExpr::Symbol(def_name) = &expressions[1] {

                                        match expressions.get(2) {
                                            Some(def_value) => {

                                                let var_value = evaluate(def_value.clone(), env)?;
                                                env.set(def_name.clone(), var_value);
                                                Ok(LispExpr::Symbol("ok".to_string()))
                                            }

                                            None => Err("definition needs a value!".to_string()),
                                        }

                                    } else {
                                        Err("definition needs a name!".to_string())
                                    }
                                },

                                "'" => {
                                    match expressions.get(1) {
                                        Some(expr) => Ok(expr.clone()),
                                        None => Err("no expression found to return!".to_string()),
                                    }
                                },

                                "+" => {

                                    let left = evaluate(expressions[1].clone(), env)?;
                                    let right = evaluate(expressions[2].clone(), env)?;

                                    match (left, right) {
                                        (LispExpr::Symbol(l), LispExpr::Symbol(r)) => {
                                            let l_val = l.parse::<i32>().map_err(|_| "expected an integer".to_string())?;
                                            let r_val = r.parse::<i32>().map_err(|_| "expected an integer".to_string())?;
            
                                            let result = l_val + r_val;

                                            Ok(LispExpr::Symbol(result.to_string()))
                                        },
                                        _ => Err("arithmetic operations require integer symbols".to_string()),
                                    }

                                },

                                _ => Err("unknown symbol!".to_string())
                            }
                        },
                        _ => Ok(expr.clone())
                    }
                },
                None => Ok(expr),
            }
        }
        LispExpr::Symbol(_) => Ok(expr),
    }
}

fn parse_tokens(tokens: &[String]) -> Result<(LispExpr, &[String]), String> {
    let (token, rest) = tokens.split_first().ok_or("Unexpected end of input")?;
    match token.as_str() {
        "(" => {
            let mut list_items = Vec::new();
            let mut inner_tokens = rest;
            while inner_tokens.get(0) != Some(&")".to_string()) {
                let (next_expr, remaining_tokens) = parse_tokens(inner_tokens)?;
                list_items.push(next_expr);
                inner_tokens = remaining_tokens;
            }
            // Skip the closing ")"
            Ok((LispExpr::List(list_items), &inner_tokens[1..]))
        }
        ")" => Err("Unexpected closing parenthesis".to_string()),
        _ => Ok((LispExpr::Symbol(token.clone()), rest)),
    }
}

pub fn parse_lispeum_string(input: &str) -> Result<LispExpr, String> {
    let tokens = tokenize(input);
    let (expr, remaining_tokens) = parse_tokens(&tokens)?;
    if !remaining_tokens.is_empty() {
        Err("Unexpected tokens after valid expression".to_string())
    } else {
        Ok(expr)
    }
}

fn tokenize(input: &str) -> Vec<String> {
    input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(String::from)
        .collect()
}
