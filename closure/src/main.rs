use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    // ゆ っ く り 計 算 し ま す
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}



fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            // 今 日 は{}回 腕 立 て 伏 せ を し て く だ さ い ！
            "Today, do {} pushups!",
            expensive_closure.value(intensity)
        );
        println!(
            // 次 に 、{}回 腹 筋 を し て く だ さ い ！
            "Next, do {} situps!",
            expensive_closure.value(intensity + 1)  // cacheしてるので間違ってる結果が...
        );
    } else {
        if random_number == 3 {
            // 今 日 は 休 憩 し て く だ さ い ！ 水 分 補 給 を 忘 れ ず に ！
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                // 今 日 は 、{}分 間 走 っ て く だ さ い ！
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );


    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // こ こ で は 、xを 使 用 で き ま せ ん: {:?}
    //println!("can't use x here: {:?}", x); // moveがあるので、xがequal_to_xに取られててコンパイルエラーんある
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
