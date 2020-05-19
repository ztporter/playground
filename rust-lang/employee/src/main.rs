mod employee_db;
mod menu;


use std::process;
use employee_db::EmployeeDatabase;


fn main() {
    let mut employee_db = employee_db::new();
    loop {
        menu::print_menu_options();
        let selection: u32 = menu::get_menu_selection();
        process_selection(selection, &mut employee_db);
    }
}


fn process_selection(selection: u32, employee_db: &mut EmployeeDatabase) {
    match selection {
        1 => add_new_employee(employee_db),
        2 => list_department(employee_db),
        3 => employee_db.print_database(),
        4 => process::exit(0),
        _ => menu::print_error("invalid selection!")
    }
}


fn add_new_employee(employee_db: &mut EmployeeDatabase) {
    let name = menu::get_input("enter employee name");
    let department = menu::get_input("enter employee department");
    employee_db.add_employee(&name, &department);
}


fn list_department(employee_db: &mut EmployeeDatabase) {
    let department = menu::get_input("enter employee department");
    if let Err(message) = employee_db.print_department(&department) {
        menu::print_error(message);
    }
}
