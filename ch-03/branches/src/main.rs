#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // numberの値は、{}です
    println!("The value of number is: {}", number);
}
