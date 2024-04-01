use std::collections::HashMap;

use super::expression::Expr;

#[derive(Debug)]
pub struct Environment {
    pub data: HashMap<String, Expr>,
}

impl Environment {
    pub fn new() -> Self {
        let env = Self { data: HashMap::new() };
        env
    }

    pub fn set(&mut self, name: String, value: Expr) {
        self.data.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Expr> {
        self.data.get(name)
    }
}