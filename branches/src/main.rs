fn main() {
    println!("Hello, world!");

    let number = 5;

    if number != 0 {
        println!("condition true");
    } else {
        println!("cond false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    // numberの 値 は 、{}で す
    println!("The value of number is: {}", number);

    //let number = if condition { 5 } else { "six" }; // 型が合わない
    //let number = if condition { 5 } else { 6.0 }; // 型が合わない
    //println!("The value of number is: {}", number);
}
