fn main() {
    let n = 1000000;
    let primes = sieve(n);
    println!("{}", primes[10000]);
}

fn sieve(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n+1];
    let mut primes = Vec::new();

    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=n{
        if is_prime[i]{
            primes.push(i);
            let mut multiple = i * i;
            while multiple <= n {
                is_prime[multiple] = false;
                multiple +=i;
            }
        }
    }
    primes
}
