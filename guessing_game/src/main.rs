extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    /*
      コメント
    /* ネストできるよー */
     */
    println!("Guess the number!"); // 数 を 当 て て ご ら ん

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("secret number: {}", secret_number); // debug mode!

    loop {
        println!("Please input your guess."); // ほ ら 、予 想 を 入 力 し て ね
        let mut guess = String::new();

        let readsize = io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Please type number: {}", err);
                continue
            },
        };


        println!("You guessed: {}", guess);
        println!("input size: {}", readsize);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
        }
    }
}
