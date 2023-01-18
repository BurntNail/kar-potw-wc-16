use crate::{Int};
use std::fmt::{Debug};

#[derive(Copy, Clone)]
pub struct El<const N: usize> ([Int; N]);

impl<const N: usize> El<N> {
    pub fn new(es: [Int; N]) -> Self {
        Self (es)
    }
    pub fn from_string (s: String) -> Self {
        assert!(s.len() == N);

        todo!()
    }


    pub fn eval (&self) -> bool {
        let expected = self.0.iter().map(ToString::to_string).collect::<String>().parse::<Int>().unwrap();

        let start = self.0.iter().sum::<Int>();
        let end = self.0.iter().map(|x| x * x).sum::<Int>();


        start * end * end == expected
    }
}

impl<const N: usize> Debug for El<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("El").field(&self.0.iter().map(ToString::to_string).collect::<String>().parse::<Int>().unwrap()).finish()
    }
}