use self::{expression::Expr, parser::parse, tokenizer::tokenize};

mod environment;
mod evaluator;
mod expression;
mod parser;
mod special;
mod tokenizer;

// pub fn evaluate(expr: LispExpr, env: &mut Environment) -> Result<LispExpr, String> {
//     match expr {
//         LispExpr::List(ref expressions) => {

//             match expressions.get(0) {
//                 Some(first_expression) => {

//                     match first_expression {
//                         LispExpr::Symbol(first_symbol) => {

//                             println!("first_symbol -> {}", first_symbol);

//                             match &first_symbol[..] {
//                                 "def" => {
                                    
//                                     if let LispExpr::Symbol(def_name) = &expressions[1] {

//                                         match expressions.get(3) {
//                                             Some(_) => {
//                                                 let var_value = LispExpr::List(expressions[2..].to_vec());
//                                                 env.set(def_name.clone(), var_value);
//                                                 Ok(LispExpr::Symbol("ok".to_string()))
//                                             }

//                                             None => {

//                                                 match expressions.get(2) {
//                                                     Some(only_value) => {
//                                                         env.set(def_name.clone(), only_value.clone());
//                                                         Ok(LispExpr::Symbol("ok".to_string()))
//                                                     },
//                                                     None => Err("definition needs a value!".to_string()),
//                                                 }
//                                             } 
//                                         }

//                                     } else {
//                                         Err("definition needs a name!".to_string())
//                                     }
//                                 },

//                                 "'" => {
//                                     match expressions.get(1) {
//                                         Some(expr) => Ok(expr.clone()),
//                                         None => Err("no expression found to return!".to_string()),
//                                     }
//                                 },

//                                 "+" => {

//                                     let left = evaluate(expressions[1].clone(), env)?;
//                                     print!("{}", left);
//                                     let right = evaluate(expressions[2].clone(), env)?;

//                                     match (left, right) {
//                                         (LispExpr::Symbol(l), LispExpr::Symbol(r)) => {
//                                             let l_val = l.parse::<u64>().map_err(|_| "expected an integer".to_string())?;
//                                             let r_val = r.parse::<u64>().map_err(|_| "expected an integer".to_string())?;
            
//                                             let result = l_val + r_val;

//                                             Ok(LispExpr::Symbol(result.to_string()))
//                                         },
//                                         _ => Err("arithmetic operations require integer symbols".to_string()),
//                                     }

//                                 },

//                                 _ => {

//                                     match env.get(first_symbol) {
//                                         Some(env_res) => evaluate(env_res.clone(), env),
//                                         None => Ok(expr)
//                                     }
//                                 }
//                             }
//                         },
//                         _ => Ok(expr.clone())
//                     }
//                 },
//                 None => Ok(expr),
//             }
//         }
//         LispExpr::Symbol(_) => Ok(expr),
//     }
// }

pub fn parse_lispeum_string(input: &str) -> Result<Expr, String> {
    let tokens = tokenize(input);
    let (expr, remaining_tokens) = parse(&tokens)?;
    if !remaining_tokens.is_empty() {
        Err("Unexpected tokens after valid expression".to_string())
    } else {
        Ok(expr)
    }
}


