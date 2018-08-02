fn main() {
    const Y : u32 = 1 + 2;
    println!("The value of y is: {}", Y);
    let x = 5;
    println!("The value of x is: {}", x); // xの 値 は{}で す
    let x = x * (x + 1);
    println!("The value of x is: {}", x);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    let y = tup.1;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let a = [1, 2, 3, 4, 5];
    let index = 10;
    let element = a[index];
    println!("The value of element is: {}", element); // 要 素 の 値 は{}で す

    let c = 'z';
    let z = 'ℤ ';
    let heart_eyed_cat = '😻 ';
}
