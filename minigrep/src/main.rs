use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem while parsing args: {}", err);
        process::exit(1);
    });

    let c = {
        let v: Vec<String> = vec![String::from("a"), String::from("b"), String::from("c")];
        Config::new(&v)
    };
    let c = c.unwrap_or_else(|err| {
        println!("Problem while parsing args: {}", err);
        process::exit(1)
    });


    println!("searching {}", config.query);
    println!("in file {}", config.filename);


    let mut f = File::open(config.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something wrong");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> { // こっちの方が正しい(strが'staticのlifetime)はずだが、一旦どこでコンパイルエラーになるかをみる
    //fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough argument");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
