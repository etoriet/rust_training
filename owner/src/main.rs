fn main() {
    let mut s = String::from("hello");
    //let mut s = "hello"; // コンパイルエラー
    s.push_str(", world!"); // push_str()関 数 は 、リ テ ラ ル をStringに 付 け 加 え る
    println!("{}", s); // こ れ は `hello, world!`と 出 力 す る

    /*
    let s1 = String::from("hello");
    let s2 = s1; // これだと次の行は動かない
    println!("{}, world!", s1); // コンパイルエラー! s1はs2にmoveされている
     */
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}, world!, {}", s1, s2); // 動く。s1はそのままになってる

    let s1 = "hello";
    let s2 = s1;
    println!("{}, world! {}", s1, s2); // 動く。s1はそのままになってる

    let s = String::from("hello"); // sが ス コ ー プ に 入 る
    takes_ownership(s); // sの 値 が 関 数 に ム ー ブ さ れ..
    // takes_ownership(s); // ... こ こ で は も う 有 効 で は な い
    let x = 5; // xが ス コ ー プ に 入 る
    makes_copy(x); // xも 関 数 に ム ー ブ さ れ る が 、i32はCopyな の で 、こ の 後 にxを 使ってもよい
    makes_copy(x); // ここでも有効
    println!("{}", x);

    let _s1 = gives_ownership(); // gives_ownership は 、戻 り 値 をs1に
    // ム ー ブ す る
    let s2 = String::from("hello"); // s2が ス コ ー プ に 入 る
    let _s3 = takes_and_gives_back(s2); // s2はtakes_and_gives_backに ム ー ブ さ れ
    // 戻 り 値 も_s3に ム ー ブ さ れ る

    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 値を渡すのでなく参照を渡す。さもないとタプルで返すとかやらなきゃならんくなる
    println!("The length of '{}' is {}.", s1, len);

    change(&s1);

    let mut s = String::from("hello");
    change2(&mut s);
    change2(&mut s);
    println!("{}", s);

    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // 不変参照は２つ作れる
    //let r3 = &mut s; // BIG PROBLEM: 不変参照が有効な間に可変参照を作ると死ぬ
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_stringはここで解放される

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // こ こ でsome_integerが ス コ ー プ を 抜 け る 。何 も 特 別 な こ と は な い 。

fn gives_ownership() -> String { // gives_ownership は 、戻 り 値 を呼 び 出 し た 関 数 に ム ー ブ す る

    let some_string = String::from("hello"); // some_stringが ス コ ー プ に 入 る
    some_string // some_stringが 返 さ れ 、呼 び 出 し 元 関 数 に
    // ム ー ブ さ れ る
}
// takes_and_gives_back は 、Stringを 一 つ 受 け 取 り 、返 す 。
fn takes_and_gives_back(a_string: String) -> String { // a_stringが ス コ ー プ に 入る 。
    println!("{}", a_string); // printlnにも渡しているように見えるが、、、なぜか所有権を維持している
    a_string // a_stringが 返 さ れ 、呼 び 出 し 元 関 数 に ム ー ブ さ れ る
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(_some_string: &String) {
    //some_string.push_str(", world"); // 参照しているものは不変なのでエラーになる
}

fn change2(some_string: &mut String) {
    some_string.push_str(", world"); // 参照しているものは不変なのでエラーになる
}

//ダングリング参照: 作成できない
/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s
} // ここで s のスコープを抜けるので &s は無効な参照となってしまう
*/
