#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

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

}
