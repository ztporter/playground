use gui::Draw;

#[derive(Debug)]
pub struct SelectButton {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl SelectButton {
    pub fn new(width: u32, height: u32) -> Box<SelectButton> {
        Box::new(SelectButton { width, height, options: vec![]})
    }

    pub fn add_option(&mut self, option: &str) {
        self.options.push(String::from(option));
    }
}

impl Draw for SelectButton {
    fn draw(&self) {
        println!("{:?}", self);
    }
}
use gui::{Screen, Button};

fn main() {
    let mut select = SelectButton::new(75, 10);
    select.add_option("Yes");
    select.add_option("Maybe");
    select.add_option("No");

    let button = Button::new(50, 10, "OK");

    let mut screen = Screen::new();
    screen.add_component(select);
    screen.add_component(button);

    screen.run()
}
