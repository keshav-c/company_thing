use std::{collections::HashMap, io};

#[derive(Debug)]
struct Person {
    name: String,
    department: String,
}

#[derive(Debug)]
enum Command {
    Add(Person),
    Remove(Person),
    List,
    Exit,
    Error,
}

#[derive(Debug)]
struct Company {
    employees: Vec<String>,
    department_employees: HashMap<String, Vec<String>>,
    employee_departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn add(&mut self, person: Person) {
        let Person { name, department } = person;
        if !self.employees.contains(&name) {
            self.employees.push(name.clone());
            self.employees.sort();
        }
        let department_employees = self
            .department_employees
            .entry(department.clone())
            .or_insert(Vec::new());
        if !department_employees.contains(&name) {
            department_employees.push(name.clone());
            department_employees.sort();
        }
        let employee_departments = self.employee_departments.entry(name).or_insert(Vec::new());
        if !employee_departments.contains(&department) {
            employee_departments.push(department);
            employee_departments.sort();
        }
    }

    fn remove(&mut self, person: Person) {
        let Person { name, department } = person;
        if let Some(employees) = self.department_employees.get_mut(&department) {
            if let Ok(i) = employees.binary_search(&name) {
                employees.remove(i);
            }
        }
        if let Some(departments) = self.employee_departments.get_mut(&name) {
            if let Ok(i) = departments.binary_search(&department) {
                departments.remove(i);
            }
        }
        // TODO: Remove employee if they are not in any departments
        // TODO: Remove department if it has no employees
    }
}

fn main() {
    let mut company = Company {
        employees: Vec::new(),
        department_employees: HashMap::new(),
        employee_departments: HashMap::new(),
    };

    loop {
        println!("Enter command.");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");

        let command: Command = parse_input(command);
        println!("{:#?}", command);

        match command {
            Command::Add(person) => {
                company.add(person);
                println!("Company {:#?}", company);
            }
            Command::Remove(person) => {
                println!("Not implemented");
            }
            Command::Exit => {
                println!("Exiting");
                break;
            }
            _ => {
                println!("Not implemented");
            }
        }
    }
}

fn parse_input(input: String) -> Command {
    let tokens: Vec<&str> = input.trim().split_whitespace().collect();
    let command_key = tokens[0].to_ascii_lowercase();
    let command_key = command_key.as_str();

    match command_key {
        "add" => {
            let remaining_tokens = tokens[1..].to_vec();
            let end_i = get_name_end(&remaining_tokens, "to");
            match end_i {
                0 => return Command::Error,
                _ => {
                    let name = remaining_tokens[0..end_i].join(" ");
                    let dept_start = end_i + 1;
                    match remaining_tokens.get(dept_start) {
                        None => Command::Error,
                        Some(_) => {
                            let department = remaining_tokens[dept_start..].join(" ");
                            Command::Add(Person { name, department })
                        }
                    }
                }
            }
        }
        "remove" => {
            let remaining_tokens = tokens[1..].to_vec();
            let end_i = get_name_end(&remaining_tokens, "from");
            match end_i {
                0 => return Command::Error,
                _ => {
                    let name = remaining_tokens[0..end_i].join(" ");
                    let dept_start = end_i + 1;
                    match remaining_tokens.get(dept_start) {
                        None => Command::Error,
                        Some(_) => {
                            let department = remaining_tokens[dept_start..].join(" ");
                            Command::Remove(Person { name, department })
                        }
                    }
                }
            }
        }
        "list" => Command::List,
        "exit" => Command::Exit,
        _ => Command::Error,
    }
}

fn get_name_end(tokens: &Vec<&str>, end_word: &str) -> usize {
    for (i, &token) in tokens.iter().enumerate() {
        if token == end_word {
            return i;
        }
    }
    0
}
