fn main() {
    let mut s = String::from("hello");
    //let mut s = "hello"; // コンパイルエラー
    s.push_str(", world!"); // push_str()関 数 は 、リ テ ラ ル をStringに 付 け 加 え る
    println!("{}", s); // こ れ は `hello, world!`と 出 力 す る

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1); // コンパイルエラー! s1はs2にmoveされている
}
