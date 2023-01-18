#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use element::El;
use rayon::prelude::*;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

mod element;

pub type Int = i128;

fn main() {
    let mut first_3 = vec![];
    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                first_3.push((i, j, k));
            }
        }
    }

    let total = first_3.len();
    let left = Arc::new(AtomicUsize::new(total));
    let total = total as f32;

    let beep_boop: Vec<String> = first_3
        .into_par_iter()
        .map_with(left, |left, (a, b, c)| {
            let st_first_3 = format!("{a}{b}{c}");
            let v: Vec<String> = (0..100_000)
                .map(|rest| format!("{rest:05}") + &st_first_3)
                .map(|s| El::from(s.as_str()))
                .filter(El::eval)
                .map(|el| el.to_string())
                .collect();
            eprintln!(
                "{:2.2}% left",
                left.fetch_sub(1, Ordering::SeqCst) as f32 / total * 100.0
            );
            v
        })
        .flatten()
        .collect();

    println!("{}", beep_boop.join("\n"));
}
