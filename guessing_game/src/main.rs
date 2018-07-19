use std::io;

fn main() {
    /*
      コメント
    /* ネストできるよー */
     */
    println!("Guess the number!"); // 数 を 当 て て ご ら ん
    println!("Please input your guess."); // ほ ら 、予 想 を 入 力 し て ね
    let mut guess = String::new();

    let readsize = io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);
    println!("input size: {}", readsize);
}
