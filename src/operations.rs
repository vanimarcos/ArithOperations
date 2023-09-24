pub fn add(a: u32, b: u32) -> u64 {
    return (a + b).into();
}

pub fn sub(a: u32, b: u32) -> u64 {
    return (a - b).into();
}

pub fn multiply(a: u32, b: u32) -> u64 {
    return (a * b).into();
}

pub fn divide(a: u32, b: u32) -> f64 {
    if b == 0 {
        panic!("Arithmetic Exception Division by Zero");
    }
    return (a / b).into();
}
