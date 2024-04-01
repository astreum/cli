use std::collections::HashMap;
mod adder;
mod define;
use super::{environment::Environment, expression::Expr};

type FnPtr = fn(Vec<Expr>, &mut Environment, &SpecialFunctions) -> Result<Expr, String>;

pub struct SpecialFunctions {
    pub functions: HashMap<String, FnPtr>,
}

impl SpecialFunctions {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        functions.insert("def".to_string(), define::define as FnPtr);
        functions.insert("+".to_string(), adder::add as FnPtr);
        Self { functions }
    }

    pub fn register(&mut self, name: String, function: FnPtr) {
        self.functions.insert(name, function);
    }

    pub fn call(&self, name: &str, args: Vec<Expr>, env: &mut Environment) -> Result<Expr, String> {
        if let Some(function) = self.functions.get(name) {
            // Now passing `self` as a reference to `SpecialFunctions`
            function(args, env, self)
        } else {
            Err(format!("Function '{}' not found", name))
        }
    }
}