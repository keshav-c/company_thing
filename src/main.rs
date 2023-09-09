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
    List(String),
    Error(String),
    Exit,
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

        // why write like this? We assume no data consistency.
        // If a department exists and the employee is in it, we remove the employee
        // But later we don't assume the employee's department list is consistent and would also
        // contain the department we just removed them from. So we do the check again:
        // If employee exists and the department is in their list, we remove the department from the list

        // Here the problem of using if let is that we cant show errors for invalid operations

        if let Some(employees) = self.department_employees.get_mut(&department) {
            if let Ok(i) = employees.binary_search(&name) {
                employees.remove(i);
            }
            // Remove department if it has no employees
            if employees.is_empty() {
                self.department_employees.remove(&department);
            }
        }
        if let Some(departments) = self.employee_departments.get_mut(&name) {
            if let Ok(i) = departments.binary_search(&department) {
                departments.remove(i);
            }
            // Remove employee if they are not in any departments. No need to remove department.
            if departments.is_empty() {
                if let Ok(i) = self.employees.binary_search(&name) {
                    self.employees.remove(i);
                }
            }
        }
    }

    fn list(&self, which: String) {
        // TODO: move presentation logic out of company struct and just return the list with some metadata
        match which.as_str() {
            "all" => {
                if self.employees.is_empty() {
                    println!("No Employees");
                } else {
                    println!("All Employees");
                    println!("-------------");
                    for employee in &self.employees {
                        println!("{}", employee);
                    }
                    println!("-------------");
                }
            }
            _ => match self.department_employees.get(&which) {
                None => println!("No Department found"),
                Some(employees) => {
                    println!("{} department", which);
                    println!("-------------");
                    for employee in employees {
                        println!("{}", employee);
                    }
                    println!("-------------");
                }
            },
        }
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

        match command {
            Command::Add(person) => {
                company.add(person);
                println!("OK");
            }
            Command::Remove(person) => {
                company.remove(person);
                println!("OK");
            }
            Command::List(which) => {
                company.list(which);
            }
            Command::Exit => {
                println!("Exiting");
                break;
            }
            Command::Error(message) => {
                println!("{}", message);
            }
        }
    }
}

fn parse_input(input: String) -> Command {
    let tokens: Vec<&str> = input.trim().split_whitespace().collect();
    let command_key = tokens[0].to_ascii_lowercase();
    let command_key = command_key.as_str();

    match command_key {
        "add" => match tokens.get(1) {
            None => return Command::Error(String::from("No name provided")),
            Some(_) => {
                let remaining_tokens = tokens[1..].to_vec();
                let end_i = get_name_end(&remaining_tokens, "to");
                match end_i {
                    0 => return Command::Error(String::from("Usage: ADD <name> TO <department>")),
                    _ => {
                        let name = remaining_tokens[0..end_i].join(" ");
                        let dept_start = end_i + 1;
                        match remaining_tokens.get(dept_start) {
                            None => Command::Error(String::from("No department provided")),
                            Some(_) => {
                                let department = remaining_tokens[dept_start..].join(" ");
                                Command::Add(Person { name, department })
                            }
                        }
                    }
                }
            }
        },
        "remove" => match tokens.get(1) {
            None => return Command::Error(String::from("No name provided")),
            Some(_) => {
                let remaining_tokens = tokens[1..].to_vec();
                let end_i = get_name_end(&remaining_tokens, "from");
                match end_i {
                    0 => {
                        return Command::Error(String::from(
                            "Usage: REMOVE <name> FROM <department>",
                        ))
                    }
                    _ => {
                        let name = remaining_tokens[0..end_i].join(" ");
                        let dept_start = end_i + 1;
                        match remaining_tokens.get(dept_start) {
                            None => Command::Error(String::from("No department provided")),
                            Some(_) => {
                                let department = remaining_tokens[dept_start..].join(" ");
                                Command::Remove(Person { name, department })
                            }
                        }
                    }
                }
            }
        },
        "list" => match tokens.get(1) {
            None => return Command::List(String::from("all")),
            Some(_) => {
                let remaining_tokens = tokens[1..].to_vec();
                let department = remaining_tokens.join(" ");
                Command::List(department)
            }
        },
        "exit" => Command::Exit,
        _ => Command::Error(String::from("Invalid command")),
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
