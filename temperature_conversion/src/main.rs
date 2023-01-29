use std::io;

fn main() {
    println!("Welcome to the tempeerature conversion!!!!");
    println!("There will be temperatures like celcius and fharenheite");

    println!("1. C-->F \n2. F-->C \nPlease Enter your choice");

    let mut user_choice = String::new();

    io::stdin()
        .read_line(&mut user_choice)
        .expect("failed to read user choice");

    println!("Enter the temperature");

    let mut user_temperature = String::new();

    io::stdin()
        .read_line(&mut user_temperature)
        .expect("failed to read the temperature");

    temp_conversion(user_temperature, user_choice);
}

fn temp_conversion(temperature: String, choice: String) {
    let num: i32 = temperature
        .trim()
        .parse()
        .expect("Failed to convert the user_temperature to number");

    let choice: i32 = choice
        .trim()
        .parse()
        .expect("Failed to convert the user_choice to number");

    if choice == 1 {
        println!("celcius_to_fharenheit ===> {}", (((9 / 5) * (num)) + 32));
    } else if choice == 2 {
        println!("fharenheit_to_celcius ===> {}", ((5 / 9) * (num - 32)));
    } else {
        println!("Invalid choice");
    }
}
