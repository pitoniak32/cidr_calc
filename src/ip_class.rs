use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum IpClass {
    A,
    B,
    C,
    D,
    E,
}

impl Display for IpClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
