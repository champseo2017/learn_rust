use std::io;
/* 
 input and then print the result as output, 
 we need to bring the io input/output library into scope.

 The io library comes from the standard library std
*/

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read line");
    println!("You guessed: {guess}")
}