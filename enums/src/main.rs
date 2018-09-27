enum IpAddrKind {
    V4,
    V6
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // pattern match, bind
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // コメントアウトすると non-exhaustive patterns でコンパイルエラー
        Some(i) => Some(i + 1)
    }
}

fn main() {
    let v4 = IpAddrKind::V4;
    let v4 = IpAddr::V4(127,0,0,1);

    let coin = Coin::Quarter(UsState::Alaska);
    println!("cent: {}", value_in_cents(coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {:?}", six);
    println!("none: {:?}", none);

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
