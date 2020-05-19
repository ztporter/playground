pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn new() -> Box<Screen> {
        Box::new(Screen { components: vec![] })
    }

    pub fn add_component(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }

    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{:?}", self);
    }
}

impl Button {
    pub fn new(width: u32, height: u32, label: &str) -> Box<Button> {
        Box::new(Button { width, height, label: String::from(label)})
    }
}
