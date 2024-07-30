fn main() {
    let mut vec:Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(1);
    let mut i = 1;
    let mut summ = 0;

    while vec[i] <= 4000000 {
        vec.push(vec[i] + vec[i-1]);
        
        i = i + 1;
    }
    vec.pop();
    println!("{:?}", vec);

    for elem in &vec{
        if elem%2 == 0{
            summ = summ + elem;
        }
    }

    println!("{}", summ)

}
