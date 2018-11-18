use std::io;
use std::io::Write;

mod stats;
mod piglatin;
mod employee;

fn main() {
    loop {
        // switch problems
        let command = read_command("command: ");
        let command: &str = &command;
        //println!("command: {}", command);
        if command == "exit" || command == "" {
            println!("Bye");
            return;
        }

        match command {
            "1" => problem1(),
            "2" => problem2(),
            "3" => problem3(),
            s => println!("unknown command: {}", s),
        }
    }
}

fn read_command(s: &str) -> String {
    let mut command = String::new();
    loop {
        print!("{}", s);
        match io::stdout().flush() {
            Err(e) => {
                println!("input error: {:?}", e);
                continue;
            },
            Ok(_) => {},
        }
        match io::stdin().read_line(&mut command) {
            Ok(_) => {
                return String::from(command.trim());
            },
            Err(e) => {
                println!("input error: {:?}", e);
                continue;
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

    println!("min/max {} / {}", stats::min(&parsed), stats::max(&parsed));
    println!("mean/median/mode {:.3} / {:.3} / {}",
        stats::mean(&parsed),
        stats::median(&parsed),
        stats::mode(&parsed)
    );
}

// problem 2: input word and output pig latin
fn problem2() {
    let mut input = String::new();
    println!("word?: ");
    let read = io::stdin().read_line(&mut input);
    if read.is_err() {
        println!("{:?}", read.unwrap_err());
        return;
    }

    for i in input.trim().split_whitespace() {
        println!("{} -> {}", i, piglatin::piglatin(i));
    }
}

// TODO problem 3: text interface of employee administration. impl add/list commands
fn problem3() {
    let mut db = employee::new();

    loop {
        // switch commands
        let command_line = read_command("db> ");
        let command_line: &str = &command_line;
        if command_line == "exit" || command_line == "" {
            println!("Bye");
            return;
        }
        let command: Vec<&str> = command_line.split_whitespace().collect();

        match command[0] {
            "Add" => db = employee::add(db, command[1..].to_vec()),
            "List" => employee::list(&db, command[1..].to_vec()),
            s => println!("unknown command: {}", s),
        }
    }

}