use arith_operations::operations::{add, sub, multiply, divide};

fn main() {
    println!("Hello, world!");

    let a = 10;
    let b = 2 ;

    println!("the result of {} added by  {} is {}", a, b, add(a, b));
    println!("the result of {} subtracted by {} is {}", a, b, sub(a, b));
    println!("the result of {} multiplied by {} is {}", a, b, multiply(a, b));
    println!("the result of {} divided by {} is {}", a, b, divide(a, b));
}

