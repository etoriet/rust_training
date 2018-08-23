fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1
    };


    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter().rev() {
        println!("the value is: {}", element);
    }

    let range = 1..10;
    for number in range.rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");


    /*
    loop {
        println!("Hello, world!");
    }
     */
}
