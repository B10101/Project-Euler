use rayon::prelude::*;

fn main() {
    let largest_palindrome = (100..=999).into_par_iter()
        .flat_map(|i| (i..=999).into_par_iter().map(move |j| i *j))
        .filter(|&product| is_palindrome(product))
        .max()
        .unwrap();

    println!("Largest palindrome: {}", largest_palindrome)
}

fn is_palindrome(num: u32) -> bool{
    let s = num.to_string();
    s.chars().eq(s.chars().rev())
}
