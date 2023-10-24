fn main() {
    for contador in 1..=100{
        if contador % 3 == 0 && contador % 5 ==0{
            println!("fizzbuzz");
        }
        else{
            if contador % 3 == 0{
                println!("fizz");
            }
            else if contador % 5 ==0{
                println!("buzz");
            }
            else {
                println!("{}",contador);
            }
        }
    }
}
