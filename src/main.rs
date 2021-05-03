use std::io;

fn main() {
    println!("Guess the number!");

    // prompt
    println!("Please inptu your guess."); 
    let mut guess = String::new();

    // io::stdin().read_line(&mut guess)
    //     .expect("Failed to read line");

    // こっちでも動く
    // let stdin = io::stdin();
    // stdin.read_line(&mut guess)
    //     .expect("failed to read line.");

    // Result を明示的にばらして扱う。これでも動くが、 match arm の Ok, Err の引数のバインドはいらないので
    // unused variable の警告が出てしまう。
    let ret = io::stdin().read_line(&mut guess);
    match ret {
        Ok(sz) => {
            println!("You guessed: {}", guess);
        }
        Err(e) => {
            panic!("Failed to read line");
        }
    }

    // println!("You guessed: {}", guess);
}
