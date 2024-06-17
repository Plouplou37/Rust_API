use std::process::Output;

pub fn option_example(x: i32) -> Option<String> {
    match x > 2 {
        true => Some(String::from("result")),
        false => None,
    }
}

/*enum Result<T, E> {
    Ok(T),
    Err(E),
}*/
enum Custom<T> {
    EXAMPLE(T),
    SAMPLE(T),
}

struct Rectangle<T> {
    height: T,
    width: T,
}

struct Cube<T> {
    height: T,
    width: T,
    lenght: T,
}

pub fn struct_example() {
    let r1 = Rectangle {
        height: 1,
        width: 5,
    };
    let c1 = Cube {
        height: 1,
        width: 5,
        lenght: 10,
    };
}

fn mul_of_number<T: std::ops::Mul<Output = T>>(num1: T, num2: T) -> T {
    num1 * num2
}

fn main() {
    println!("{}", mul_of_number(2, 10))
}
