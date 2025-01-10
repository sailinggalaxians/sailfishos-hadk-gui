use std::io;

fn main() {
    println!("Choose your level of computer skills:");

    let mut val: String = String::new();

    println!("1. Beginner");
    println!("2. Somewhere in between");
    println!("3. Advanced");

    io::stdin().read_line(&mut val).expect("Failed to read input");

    let val: u32 = val.trim().parse().expect("Please enter a number");

    match val {
        1 => println!("You chose Beginner"),
        2 => println!("You chose Somewhere in between"),
        3 => println!("You chose Advanced"),
        _ => println!("Invalid choice! Please select a valid option"),
    }
}
