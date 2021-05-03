use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // ThreadRng という、 thread local (?) な乱数生成器を作ってレンジ条件つきで乱数生成しているぽい。
    // Rng はトレイト (≒ インタフェース) らしい。これがスコープにいないとメソッド(メソッド?)を利用できないとかなんとか書いてある
    // 
    // トレイトはインタフェースに近いモノ、として認識しているので、まだここの話に関してはちょっと腑に落ちない。
    // use を抜くと `gen_range` が使えなくなるが、なぜすでにインスタンス化したオブジェクトのメソッドが use しないと使えないのか？？
    // このへん、　Python や JavaScript の感覚ではちょっと納得できる理屈がわからないのでカルチャーショック
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // prompt
    println!("Please inptu your guess."); 
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    println!("The secret number is: {}", secret_number);
}
