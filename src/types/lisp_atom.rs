use std::fmt;

#[derive(Debug, PartialEq)]
pub enum LispAtom {
    Int(i64),
    Float(f64),
    String(String),
    Symbol(String),
}

impl fmt::Display for LispAtom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LispAtom::Int(e) => write!(f, "{}", e),
            LispAtom::Float(e) => write!(f, "{}", e),
            LispAtom::String(e) => write!(f, "{}", e),
            LispAtom::Symbol(e) => write!(f, "{}", e),
        }
    }
}

impl From<i64> for LispAtom {
    fn from(int: i64) -> Self {
        LispAtom::Int(int)
    }
}

impl From<f64> for LispAtom {
    fn from(float: f64) -> Self {
        LispAtom::Float(float)
    }
}

impl From<&str> for LispAtom {
    fn from(string: &str) -> Self {
        LispAtom::String(string.to_string())
    }
}

impl From<String> for LispAtom {
    fn from(string: String) -> Self {
        LispAtom::String(string)
    }
}

impl LispAtom {
    pub fn new_symbol<T>(str: T) -> Self
    where
        T: Into<String>,
    {
        LispAtom::Symbol(str.into())
    }
}
