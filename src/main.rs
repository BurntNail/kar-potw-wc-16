use crate::tim::ScopedTimer;
use crossbeam::channel::{unbounded, Receiver, Sender};
use matrix_element::El;
use rayon::prelude::*;
use std::thread;

mod matrix_element;
mod tim;

pub type Int = i128;


fn main() {
    let mut first_2 = vec![];
    for i in 0..10 {
        for j in 0..10 {
            first_2.push((i, j));
        }
    }

    first_2.into_par_iter().for_each(|(a, b)| {
        for c in 0..10 {
            for d in 0..10 {
                for e in 0..10 {
                    for f in 0..10 {
                        let el = El::new([a, b, c, d, e, f]);
                        if el.eval() {
                            println!("{el:?}");
                        }  
                    }  
                }
            }
        }
    });
}
