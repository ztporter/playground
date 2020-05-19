use std::collections::HashMap;

pub struct EmployeeDatabase {
    map: HashMap<String, Vec<String>>
}

impl EmployeeDatabase {
    pub fn add_employee(&mut self, name: &String, department: &String) {
        let department_employees = self.map.entry(department.to_string()).or_insert(Vec::<String>::new());
        department_employees.push(name.to_string());
    }

    pub fn print_department(&mut self, department: &String) -> Result<(), &str> {
        match self.map.get_mut(department) {
            Some(mut employee_list) => {
                println!("\n{} department:", department);
                EmployeeDatabase::print_employees(&mut employee_list);
                return Result::Ok(());
            },
            None => return Result::Err("no such department")
        }
    }

    pub fn print_database(&mut self) {
        let department_list = self.get_sorted_department_list();
        for department in department_list {
            let _ = self.print_department(&department);
        }
    }

    fn get_sorted_department_list(&self) -> Vec<String> {
        let mut department_list: Vec<String> = Vec::new();
        for (key, _value) in &(self.map) {
            department_list.push(key.to_owned());
        }
        department_list.sort();
        department_list
    }

    fn print_employees(employee_list: &mut Vec<String>) {
        employee_list.sort();
        let mut count = 1;
        for employee in employee_list {
            println!("  {}) {}", count, employee);
            count += 1;
        }
    }
}

pub fn new () -> EmployeeDatabase {
    EmployeeDatabase {map: HashMap::<String, Vec<String>>::new()}
}
