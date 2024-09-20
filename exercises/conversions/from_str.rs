// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

//步骤:
// 1。如果提供的字符串长度为0，则应返回错误
// 2。分割给定字符串中的逗号
// 3。拆分只能返回2个元素，否则返回1
//错误
// 4。从拆分操作中提取第一个元素并将其用作名称
// 5。从拆分操作中提取另一个元素并将其解析为
// ' usize '作为年龄，类似' "4".parse::<usize>() '
// 6。如果在提取姓名和年龄时出了问题，就会出现错误
//应该返回
//如果一切正常，则返回Person对象的Result
//
//作为题外话:' Box<dyn Error> '实现' From<&'_ str> '。这意味着如果
//如果你想返回一个字符串错误消息，你可以使用
//返回' Err("my error message".into()) '。

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s == "" { return Err(ParsePersonError::Empty); }
        let mut p = Person { name: String::new(), age: 0 };
        let parts: Vec<&str> = s.split(",").collect();
        if parts.len() != 2 { return Err(ParsePersonError::BadLen); }

        let name = parts.get(0).unwrap().to_string();
        let age = parts.get(1).unwrap().to_string();
        if name == "" { return Err(ParsePersonError::NoName); }

        match age.parse::<usize>() {
            Ok(v_age) => {
                p.age = v_age;
            }
            Err(err) => {
                return Err(ParsePersonError::ParseInt(err))
            }
        }
        p.name = name;
        Ok(p)
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
