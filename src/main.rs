use std::io;

fn main() {
    println!("Guess the number!");

    // prompt
    println!("Please inptu your guess."); 
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    // こっちでも動く
    // let stdin = io::stdin();
    // stdin.read_line(&mut guess)
    //     .expect("failed to read line.");

    println!("You guessed: {}", guess);
}
