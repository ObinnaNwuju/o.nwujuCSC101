use std::io;

fn main() {
    let mut highest: (String, u32) = ("None".to_string(), 0);

    println!("How many candidates?");
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap(); 
    let n: u32 = n.trim().parse().unwrap();

    for _ in 0..n{
        let mut name = String::new();
        let mut exp = String::new();

        println!("Enter candidate name:");
        io::stdin().read_line(&mut name).unwrap();

        println!("Enter years of programming experience:");
        io::stdin().read_line(&mut exp).unwrap();

        let exp: u32 = exp.trim().parse().unwrap();


        if exp > highest.1 {
            highest = (name.trim().to_string(), exp);
        }
    }

    println!("\nThe candidate with the highest programming experience is {} with {} years.", highest.0, highest.1);
}