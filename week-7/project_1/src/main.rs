use std::io;

fn main() {
    println!("Choose what you want to be calculated:");
    println!("1. Area of Trapezium
              2. Area of Rhombus
              3. Area of Parallelogram
              4. Area of Cube
              5. Volume of Cylinder");

    println!("For Trapezium type T
              For Rhombus type R
              For Parallelogram type P
              For Cube type Cu
              For Cylinder type C");

    loop {
        let mut input = String::new();
        println!("Enter the Character:");
        io::stdin().read_line(&mut input).expect("Not a valid string");
        let input1 = input.trim();

        if !["T", "R", "P", "Cu", "C"].contains(&input1) {
            println!("Please enter a valid command");
            continue;
        }

        match input1 {
            "T" => {
                let (base1, base2, height) = get_trapezium_dimensions();
                let area = height / 2.0 * (base1 + base2);
                println!("Area of Trapezium = {}", area);
            }
            "R" => {
                let (diagonal1, diagonal2) = get_rhombus_dimensions();
                let area = 0.5 * diagonal1 * diagonal2;
                println!("Area of Rhombus = {}", area);
            }
            "P" => {
                let (base, height) = get_parallelogram_dimensions();
                let area = base * height;
                println!("Area of Parallelogram = {}", area);
            }
            "Cu" => {
                let side = get_cube_dimension();
                let area = 6.0 * (side * side);
                println!("Area of Cube = {}", area);
            }
            "C" => {
                let (radius, height) = get_cylinder_dimensions();
                let volume = std::f64::consts::PI * radius.powi(2) * height;
                println!("Volume of Cylinder = {}", volume);
            }
            _ => {}
        }

        // Ask the user if they want to perform another calculation
        let mut continue_input = String::new();
        println!("Do you want to perform another calculation? (Y/N):");
        io::stdin().read_line(&mut continue_input).expect("Failed to read line");
        let continue_choice = continue_input.trim().to_uppercase();

        if continue_choice != "Y" {
            println!("Thank you for using the calculator. Goodbye!");
            break; 
        }
    }
}

fn get_trapezium_dimensions() -> (f64, f64, f64) {
    let (base1, base2, height) = (get_input("Enter base1: "), get_input("Enter base2: "), get_input("Enter height: "));
    (base1, base2, height)
}

fn get_rhombus_dimensions() -> (f64, f64) {
    let (diagonal1, diagonal2) = (get_input("Enter diagonal1: "), get_input("Enter diagonal2: "));
    (diagonal1, diagonal2)
}

fn get_parallelogram_dimensions() -> (f64, f64) {
    let (base, height) = (get_input("Enter base: "), get_input("Enter height: "));
    (base, height)
}

fn get_cube_dimension() -> f64 {
    get_input("Enter the length of the side: ")
}

fn get_cylinder_dimensions() -> (f64, f64) {
    let (radius, height) = (get_input("Enter radius: "), get_input("Enter height: "));
    (radius, height)
}

fn get_input(prompt: &str) -> f64 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please enter a valid number")
}