fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let large = largest(&number_list).expect("non empty");
    println!("largest: {}", large);
    assert_eq!(large, 100);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let large = largest(&number_list).expect("non empty");
    println!("largest: {}", large);
    assert_eq!(large, 6000);


    let int_p = Point { x: 1, y: 10 };
    let flo_p = Point { x: 1.0, y: 10.0 };
    // 型が違うとコンパイルできない
    // let wont_work = Point { x: 1, y: 10.0 };
    println!("int_p.x: {}", int_p.x());
    // 特化したメソッドを宣言できる
    flo_p.distance_from_origin();
    // int_p.distance_from_origin(); // i32には宣言されてないメソッドなのでコンパイルできない

    // implの型引数とメソッドの型引数は一致している必要がない
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


fn largest<T: PartialOrd + Copy>(slice: &[T]) -> Option<T> {
    if slice.len() == 0 {
        return None
    }

    let mut largest = slice[0];

    for number in slice {
        if *number > largest {
            largest = *number;
        }
    }

    Some(largest)
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}
