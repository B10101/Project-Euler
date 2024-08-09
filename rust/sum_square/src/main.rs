use rayon::prelude::*;

fn main() {
    let numbers: Vec<i32> = (1..=100).collect();
    let sum_of_squares:i32 = numbers.par_iter().map(|&x| x * x).sum();
    let sum:i32 = numbers.par_iter().sum();
    let squared:i32 = sum * sum;

    println!("{}", sum_of_squares-squared);
}
