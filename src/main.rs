use std::io;
use std::cmp::Ordering;
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

    loop {
        // prompt
        println!("Please input your guess."); 
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // match は式らしい。このへんは自分が知っている if, switch と少し勝手が異なる。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("The secret number is: {}", secret_number);
    }
}
