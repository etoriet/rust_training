use std::collections::HashMap;

pub fn new() -> HashMap<String, Vec<String>> {
    HashMap::new()
}

pub fn add(db: &mut HashMap<String, Vec<String>>, command_line: Vec<&str>) {
    println!("{:?}", command_line);
    // TODO
}

pub fn list(db: &mut HashMap<String, Vec<String>>, command_line: Vec<&str>) {
    println!("{:?}", command_line);
    // TODO
}