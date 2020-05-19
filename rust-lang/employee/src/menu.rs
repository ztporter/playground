use std::io::{self, Write};
use termion::color;


pub fn print_menu_options() {
    println!("");
    println!("**************************************************");
    println!("*        Employee Database Manager Menu          *");
    println!("**************************************************");
    println!("* 1) add new employee                            *");
    println!("* 2) list department employees                   *");
    println!("* 3) list company employees                      *");
    println!("* 4) exit application                            *");
    println!("**************************************************");
    println!("")
}


pub fn get_menu_selection() -> u32 {
    loop {
        let selection = get_input("enter selection");

        match selection.parse::<u32>() {
            Ok(num) => return num,
            Err(_) => {
                print_error("selection must be a number!");
                continue;
            }
        };
    }
}


pub fn get_input(message: &str) -> String {
        print!("{}: ", message);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("unable to read input");
        input.truncate(input.len() - 1);
        input
}


pub fn print_error(message: &str) {
    println!("{}{}{}", color::Fg(color::Red), message, color::Fg(color::AnsiValue(15)));
}
