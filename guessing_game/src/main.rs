use std::io;    // io library for user inputs
use rand::Rng;  // random number generater
use std::cmp::Ordering;  // enum: less, equal, and greater

fn main() {
    println!("Guess the number");

    // 1..101 is same as 1..=100
    let secret_number = rand::thread_rng().gen_range(1..101);
    //println!("The secret number is: {}", secret_number);

    loop{
        println!("Please input your guess");
        // let: create a variable
        // mut: the variable is mutable (immutable by default)
        // new is String type's associated function (similiar to cpp class's static function)
        let mut guess = String::new();
    
        // "&" is reference symbol
        // read_line will return io::Result, if it is Err, expect method
        // (defined on Result type) will be caused
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        // shadow variables with the same name before
        let guess : u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guess: {}", guess);
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("sao");
                break;
            }
        }
    }
}
