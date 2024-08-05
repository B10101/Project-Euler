fn prime_factors(n: u64) ->Vec<u64> {
    let mut factors:Vec<u64> = Vec::new();
    let mut num = n;

    // handling factors of 2
    while num % 2 == 0{
        factors.push(2);
        num /=2;
    }
    //factors of 3
    while num % 3 == 0{
        factors.push(3);
        num /=3;
    }

    //odd numbers from 5
    let mut factor = 5;
    while factor * factor <= num{
        while num%factor == 0{
            factors.push(factor);
            num /= factor;
        }
        factor += 2;
    }
    if num>1{
        factors.push(num);
    }

    factors

}

fn main(){
    let number = 600851475143;
    let factors = prime_factors(number);
    println!("{:?}", factors);
}

