// pub struct Guess {
//     value: i32,
// }
// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1  {
//             panic!("значение должно быть меньше или равно 100, получено {}", value);
//         }
//         else if value > 100 {
//             panic!("Значение должно быть меньше или равно 1, получено {}", value);
//         }
//         Guess { value }
//     }
// }
// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     #[should_panic(expected = "Значение должно быть меньше или равно 100")]
//     fn greater_than_100() {
//         Guess::new(200);
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() -> Result<(), String> {
//         if 2 + 2 == 4 {
//             Ok(())
//         } else {
//             Err(String::from("2 + 2 не равно 4"))
//         }
//     }
// }

fn prints_and_returns_10(a: i32) -> i32 {
    println!("Я получил значение {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}




// #[derive(Debug)]
// pub struct Rectangle {
//     length: u32,
//     width: u32,
// }
// impl Rectangle {
//     pub fn can_hold(&self, other: &Rectangle) -> bool {
//         self.length > other.length && self.width > other.width
//     }
// }
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {length: 8, width: 7};
//         let smaller = Rectangle {length: 5, width: 1};
//         assert!(larger.can_hold(&smaller));
//     }
//     #[test]
//     fn smaller_can_hold_larger() {
//         let larger = Rectangle {length: 8, width: 7};
//         let smaller = Rectangle {length: 9, width: 10};
//         assert!(smaller.can_hold(&larger));        
//     }
// }
// pub fn greeting(name: &str) -> String {
//     format!("Здравствуй,")
// }
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn greeting_name() {
//         let result = greeting("Кэрол");
//         assert!(result.contains("Кэрол"), "Приветствие не содержало имени, предоставлено значение `{}`", result);
//     }
// }