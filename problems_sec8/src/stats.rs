use ::std;
use std::collections::HashMap;

pub fn min(vec: &Vec<i32>) -> i32 {
    vec.iter().fold(std::i32::MAX, |m, &a| if a < m { a } else { m })
}
pub fn max(vec: &Vec<i32>) -> i32 {
    vec.iter().fold(std::i32::MIN, |m, &a| if a > m { a } else { m })
}

pub fn mean(vec: &Vec<i32>) -> f64 {
    let sum = vec.iter().fold(0, |m, &a| m + a);

    (sum as f64) / (vec.len() as f64)
}

pub fn median(vec: &Vec<i32>) -> f64 {
    let mut vec = vec.to_vec();
    vec.sort();
    let mid1 = vec.len() / 2;
    if vec.len() % 2 == 0 {
        ((vec[mid1 - 1] + vec[mid1]) as f64) / 2.0
    } else {
        vec[mid1] as f64
    }
}

// should return multiple values: mode may contain multiple
pub fn mode(vec: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for i in vec {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    match map.values().max() {
        Some(max_count) =>
            map.
            iter().
            filter(|&(_, v)| v == max_count).
            map(|(&&k, _)| k).
            collect(),
        None => Vec::new(),
    }
}
