#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 24, width: 20 };
        let smaller = Rectangle { length: 18, width: 18 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 24, width: 20 };
        let smaller = Rectangle { length: 18, width: 18 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn cannot_hold_self() {
        let rect = Rectangle { length: 24, width: 20 };

        assert!(!rect.can_hold(&rect));
    }

    #[test]
    #[ignore]
    fn it_adds_two() {
        assert_eq!(add_two(4), 6);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "greeting did not contain name, result was: `{}`", result);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two (a: i32) -> i32 {
    a + 2
}

pub fn greeting (name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}!", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}!", value);
        }

        Guess { value }
    }
}
