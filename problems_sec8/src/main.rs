use std::io;

fn main() {
    loop {
        // switch problems
        let command = read_command();
        let command: &str = &command;
        //println!("command: {}", command);
        if command == "exit" || command == "" {
            println!("Bye");
            return;
        }

        match command {
            "1" => problem1(),
            // TODO problem 2: input word and output pig latin
            "2" => println!("2"),
            // TODO problem 3: text interface of employee administration. impl add/list commands
            "3" => println!("3"),
            s => println!("unknown command: {}", s),
        }
    }
}

fn read_command() -> String {
    let mut command = String::new();
    loop {
        println!("command?: ");
        match io::stdin().read_line(&mut command) {
            Ok(_) => {
                return String::from(command.trim());
            },
            Err(e) => {
                println!("input error: {:?}", e);
            },
        }
    };
}

// problem 1: input numbers and output mean/median/mode
fn problem1() {
    let mut numbers = String::new();
    println!("numbers?: ");

    // std::num::ParseIntError と std::io::Error の型が合わないらしく?
    // 一度にflattenできないので分割したコードにする。
    let read  = io::stdin().read_line(&mut numbers);
    if read.is_err() {
        println!("{:?}", read.unwrap_err());
        return;
    }
    // readのOkは読み込んだバイト数で、使わないので無視する

    // parse
    // https://doc.rust-lang.org/rust-by-example/error/iter_result.html#fail-the-entire-operation-with-collect
    let parsed: Result<Vec<i32>, _> = numbers.trim().split_whitespace().map(|n| n.parse::<i32>()).collect();
    if parsed.is_err() {
        println!("{:?}", read.unwrap_err());
        return;
    }
    let parsed = parsed.unwrap();

    println!("{:?}", parsed);

    // TODO impl mean/median/mode
}
