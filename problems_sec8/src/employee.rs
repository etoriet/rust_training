use std::collections::HashMap;

pub fn new() -> HashMap<String, Vec<String>> {
    HashMap::new()
}

pub fn add(db: HashMap<String, Vec<String>>, command_line: Vec<&str>) ->  HashMap<String, Vec<String>> {
    println!("Add: {:?}", command_line);
    let employee = command_line[0];
    let department = command_line[2];

    let mut db = db;
    db.entry(department.to_string()).or_insert(Vec::new()).push(employee.to_string());
    // TODO vecでなく赤黒木とかを使う
    db.entry(department.to_string()).and_modify(|v| v.sort());
    db
}

fn output(prefix: &str, employees: &Vec<String>) {
    println!("{}:", prefix);
    for e in employees {
       println!("  {}", e);
    }
}
pub fn list(db: &HashMap<String, Vec<String>>, command_line: Vec<&str>) {
    match command_line[0] {
        "all" => {
            let mut all = Vec::new();
            for (_department, employees) in db {
                for e in employees {
                    if !all.contains(e) {
                        all.push(e.to_string());
                    }
                }
            }
            all.sort();
            output("all", &all);
        },
        "employees" => {
            let department = command_line[2];
            let employees = &db[&department.to_string()];

            output(department, employees);
        },
        _c => println!("unknown command line: {:?}", command_line),
    }
}