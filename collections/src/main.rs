#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1.0, 2.0];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    let mut v = vec![1, 2, 3, 4, 5];
    // 要素の入れ替え: i32ならcopyなので何も問題ない
    let first = v[0];
    let second = v[1];
    v[0] = second;
    v[1] = first;

    for i in &mut v { // 可変参照の取得
        *i += 50;
    }

    for i in &v { // 普遍参照の取得
        println!("{}", i);
    }


    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // 要素の入れ替え: enumは構造体なので,copyができずコンパイルエラーになる
    /*let first = &row[0];
    let second = &row[1];
    row[0] = *second;
    row[1] = *first;
     */
    // swapなら動く
    row.swap(0, 1);

    for i in &row {
        println!("{:?}", i);
    }

    let mut row = vec![
        String::from("one"),
        String::from("two"),
        String::from("three"),
    ];

    // 要素の入れ替え: Stringはむり
    /*
    let first = row[0];
    let second = row[1];
    row[0] = second;
    row[1] = first;
     */
    // swapはいける
    row.swap(0, 1);

    for i in &row {
        println!("{:?}", i);
    }

    let mut s = String::new();
    let data = "🍣 initial contents α "; // 寿司
    s = data.to_string();
    println!("{}", s);

    s.push_str("bar");
    println!("{}", s);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello,");
    let s2 = String::from("World");
    let s3 = s1 + &s2; // add method
    // println!("{}", s1); // moveされてるのでコンパイルできない
    println!("{}", s2); // 借用されてるだけなので使える
    println!("{}", s3);

    /* // これも動くが、shadowingがうまく機能してない...
    let mut s1 = String::from("Hello,");
    let s2 = String::from("World");
    s1 = s1 + &s2; // add method
    println!("{}", s1); // moveされてるけど戻ってきたので動く
    println!("{}", s2); // 借用されてるだけなので使える
     */

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); // 多分コピーしている
    println!("{}", s);
    println!("{}", s1); // 所有権を奪わない、便利

    let x = String::from("こんにちわ");
    println!("{}", x);
    let y = &x[0..3]; // 嫌すぎるけどスライスが作れる
    //let y = &x[0..2]; //文字の境界以外で区切ったらランタイムエラー
    println!("{}", y);

    for c in "どらえもん".chars() {
        println!("{}", c); // 「ど」が出力される
    }

    for c in "どらえもん".chars() { // macのファイルシステムから持ってきた文字列
        println!("{}", c); // 「と」「 ゙」が出力される
    }

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Red"), 50);
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("White"), 30);
    println!("{:?}", scores);
    println!("{}", scores.get("Red").unwrap_or(&1));

    let teams = vec![String::from("B"), String::from("Y")];
    let init_scores = vec![10, 50];
    // 型注釈しないとcollectの型が決まらない
    let scores: HashMap<&String, &i32> = teams.iter().zip(init_scores.iter()).collect();
    // 以下にするとVecになる
    //let scores: Vec<(_, _)> = teams.iter().zip(init_scores.iter()).collect();
    println!("teams: {:?}", teams); // teamsの参照は奪っていない
    println!("scores: {:?}", scores); // scoresには参照が入っている
     // ので,unwrapするには,型を合わせるために参照の参照を渡す必要がある
    println!("{}", scores.get(&String::from("B")).unwrap_or(&&0));

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{}", field_name); moveされたのでコンパイルエラーになる

    let field_name2 = String::from("Favorite color");
    println!("before update: {}", map.get(&field_name2).unwrap_or(&field_name2));
    map.insert(String::from("Favorite color"), String::from("Green"));
    println!("after update:  {}", map.get(&field_name2).unwrap_or(&field_name2));

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(10);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text =  "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
