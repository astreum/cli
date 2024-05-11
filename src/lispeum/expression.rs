use std::fmt;

#[derive(Clone, Debug)]
pub enum Expr {
    List(Vec<Expr>),
    Symbol(String),
    Integer(i64), // "123"
    String(String), // "string_example"
    Function(Vec<String>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::List(elements) => {
                let inner: Vec<String> = elements.iter().map(|e| format!("{}", e)).collect();
                write!(f, "({})", inner.join(" "))
            },
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::Integer(s) => write!(f, "{}", s),
            Expr::String(s) => write!(f, "{}", s),
            Expr::Function(params, body) => {
                let params_str = params.join(" ");
                let body_str = format!("{}", body);
                write!(f, "(fn ({}) {})", params_str, body_str)
            },
        }
    }
}