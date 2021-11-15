// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

// notes: shadowing, reference:
// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#:~:text=of%20x%20is%3A%206-,Shadowing,-is%20different%20from


fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3;
    println!("Number plus two is : {}", number + 2);
}
