pub fn piglatin(s: &str) -> String {
    let mut s = String::from(s);
    // TODO これだと1文字目までしか見てない
    let init = s.chars().next().unwrap().to_string();
    let vowel = String::from("aiueo");
    if !vowel.contains(init.as_str()) {
        s.remove(0);
        s.push_str(init.as_str());
    }
    s.push_str("ay");

    s
}