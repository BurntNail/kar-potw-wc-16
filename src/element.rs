use crate::Int;
use std::fmt::{Debug, Display};

#[derive(Clone)]
pub struct El(Vec<Int>);

impl El {
    pub fn eval(&self) -> bool {
        let expected = self.to_string().parse::<Int>().unwrap();

        let start = self.0.iter().sum::<Int>();
        let end = self.0.iter().map(|x| x * x).sum::<Int>();

        start * end * end == expected
    }
}

impl From<&str> for El {
    fn from(s: &str) -> Self {
        Self(s.chars().map(|c| Int::from(c as u8 - b'0')).collect())
    }
}

impl Display for El {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let st = self.0.iter().map(ToString::to_string).collect::<String>();
        write!(f, "{st}")
    }
}
impl Debug for El {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("El").field(&self.0).finish()
    }
}
