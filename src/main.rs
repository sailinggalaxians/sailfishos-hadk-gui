use std::io;

fn main() {
    println!("Choose your level of understanding computers:");

    let mut val: String = String::new();

    println!("1. Beginner");
    println!("2. Something higher that Beginner");
    println!("3. Professional hacker");

    io::stdin().read_line(&mut val).expect("Failed to read input");

    let val: u32 = val.trim().parse().expect("Please choose a number");

    match val {
        1 => println!("You chose Beginner"),
        2 => println!("You chose Something higher than Beginner"),
        3 => println!("You chose Professional hacker"),
        _ => println!("Error! Wrong choise! Please choose a valid option"),
    }
}
