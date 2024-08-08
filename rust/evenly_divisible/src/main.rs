fn main() {
    let start = 1;
    let end = 20;
    let result = (start..=end).fold(1,|acc,x| lcm(acc,x));
    

    println!("{}", result)
}

fn binary_gcd(mut a:u64,mut b:u64) -> u64{
    if a == 0 {
       return b;
    }
    if b == 0 {
        return a;
    }
    let mut shift = 0;
    while((a | b) & 1) == 0{
        a >>=1;
        b >>=1;
        shift +=1;
    }
    while(a&1) == 0{
        a >>= 1;
    }

    loop {
        while(b & 1) == 0{
            b >>= 1;
        }

        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;

        if b == 0 {
            break;
        }

    }
    a<<shift
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a*b) / binary_gcd(a,b)
}

