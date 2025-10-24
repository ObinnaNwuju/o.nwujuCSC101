 

 use std::io;


 fn main() {


    let mut input1 = String::new();
    let mut input2 = String::new();


    println!("Are you experienced or inexperienced");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let exp = input1.trim().to_string();


    if exp != "experienced" && exp != "inexperienced" {
    println!("Error: Invalid input. Please enter either 'experienced' or 'inexperienced'.");
    return;

    }


    if exp == "inexperienced" {
        println!("Your salary is ₦100,000");
        return;

    }


    println!("What is your age");
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let age:f64 = input2.trim().parse().expect("Invalid input");


    if exp == "experienced" && age >= 40.0 {
        println!("Your salary is ₦1,560,000")

    }


    if exp == "experienced" && age >= 30.0 && age < 40.0 {
        println!("Your salary is ₦1,480,000")

    }


    if exp == "experienced" && age < 28.0 {
        println!("Your salary is ₦1,300,000")

    }




}


