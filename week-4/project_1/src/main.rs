// Rust program to find 
// quadratic equations of numbers


use std::io;

fn main() {

    let mut input1 = String::new(); 
    let mut input2 = String::new();
    let mut input3 = String::new(); 

    println!("Enter your first number (a): ");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let a: f64 = input1.trim().parse().expect("Not a valid Number");

    println!("Enter your second number (b): ");
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let b: f64 = input2.trim().parse().expect("Not a valid Number");

    println!("Enter your third number (c): ");
    io::stdin().read_line(&mut input3).expect("Not a valid String");
    let c: f64 = input3.trim().parse().expect("Not a valid Number");

    let mut equation1: f64 = -1.0 * b;
    let equation2: f64 = (b * b - 4.0 * a * c).sqrt();
    let mut equation3: f64 = equation1 + equation2;
    let mut equation4: f64 = equation1 - equation2;
    let mut equation5: f64 = 2.0 * a;

    let root1 = equation3 / equation5;
    let root2 = equation4 / equation5;

    println!("The roots are {} and {}", root1, root2);
}
