fn main() {
    println!("Hello, world!");

    let x = 5;

    let y = another_function(x);

    println!("x is {}", x);
    println!("y is {}", y);
}

fn another_function(x: i32) -> i32 {
    println!("Another function. {}", x); // 別 の 関 数

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("x is {}", x);
    println!("y is {}", y);

    x + plus_one(y)
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
