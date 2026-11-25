// How to write tests
// Structuring test function
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Checking result with assert
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Testing Equality with assert_eq! and assert_ne!
pub fn add_two(a: u64) -> u64 {
    a + 2
}

pub fn add_two_false(a: u64) -> u64 {
    a + 3
}

// Adding custom failure message
pub fn greeting(_name: &str) -> String {
    //format!("Hello {name}!") // comment for test custom failure masse
    String::from("Hello!")
}

// Checking for panics with should_panic
#[allow(dead_code)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Structuring test function
    /*
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    */

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    /*
    #[test]
    fn another() {
        panic!("Make this test fail");
    }
    */

    // Checking result with assert
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn larger_cannot_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // Implement Testing Equality with assert_eq! and assert_ne!
    #[test]
    fn it_add_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_add_two_false() {
        let result = add_two_false(2);
        assert_ne!(result, 4);
    }

    // Implemet Adding custom failure message
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    // Implement Checking for panics with should_panic
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Using Result<T, E> in test
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
