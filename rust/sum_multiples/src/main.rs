fn main() {
    let mut sum = 0;
    for number in (1..1000).rev(){
        if (number % 3 == 0) || (number % 5 == 0){
        sum = sum + number;
        println!("{}",sum);
        }
    }

    

}
