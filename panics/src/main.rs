use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;

fn main() {
    // panicマクロによるクラッシュ
    //panic!("crash and burn");

    // 配列外アクセス: panic
    //let v = vec![1, 2, 3];
    //v[99];

    // あえて違う型を指定してエラーメッセージから型情報を読み取る
    //let f: u32 = File::open("test.txt");
    //   = note: expected type `u32`
    //              found type `std::result::Result<std::fs::File, std::io::Error>`

    let filepath = "hello.txt";
    let f = File::open(filepath);
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create(filepath) {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but problem: {:?}",
                        e
                    );
                },
            }
        },
        Err(error) => {
            // フ ァ イ ル を 開 く 際 に 問 題 が あ り ま し た
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    // panicしていい場合
    //let f = File::open("xxx.txt").unwrap(); // とりあえず取り出す
    //let f = File::open("xxx.txt").expect("Failed to open xxx.txt"); // 取り出せなかったらエラーメッセージ

    let loaded = read_username_from_file();
    println!("{:?}", loaded);
    let loaded = read_username_from_file2();
    println!("{:?}", loaded);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
