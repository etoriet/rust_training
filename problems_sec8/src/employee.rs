use std::collections::HashMap;

pub struct Employee {
    data: HashMap<String, Vec<String>>
}

fn output(prefix: &str, employees: &Vec<String>) {
    println!("{}:", prefix);
    for e in employees {
        println!("  {}", e);
    }
}


impl Employee {
    pub fn new() -> Employee {
        Employee {
            data: HashMap::new(),
        }
    }

    pub fn add(self: &mut Employee, command_line: &[&str]) {
        println!("Add: {:?}", command_line);
        let employee = command_line[0];
        let department = command_line[2];

        let db = &mut self.data;
        // TODO vecでなく赤黒木とかを使うと早くなるけどやってない
        //      vecにpushしてsortを毎回やっちゃってる
        db.entry(department.to_string()).or_insert(Vec::new()).push(employee.to_string());
        db.entry(department.to_string()).and_modify(|v| v.sort());
    }

    pub fn list(self: &Employee, command_line: &[&str]) {
        match command_line[0] {
            "all" => {
                let mut all = Vec::new();
                for (_department, employees) in &self.data {
                    for e in employees {
                        if !all.contains(e) { // TODO 線形探索している
                            all.push(e.to_string());
                        }
                    }
                }
                all.sort();
                output("all", &all);
            },
            "employees" => {
                let department = command_line[2];
                let employees = &self.data[&department.to_string()];

                output(department, employees);
            },
            _c => println!("unknown command line: {:?}", command_line),
        }
    }
}
