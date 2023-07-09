use std::io;

fn main() {
    println!("Introduce a temperature in Celsius:");
    let mut celsius_string = String::new();

    io::stdin()
        .read_line(&mut celsius_string)
        .expect("Failed to read celsius");

    let trimmed_celsius = celsius_string.trim();
    let fahrenheit: i32;

    // Convert the input to an integer
    if let Ok(parsed) = trimmed_celsius.parse::<i32>() {
        fahrenheit = (parsed * 9 / 5) + 32;
    } else {
        println!("Invalid input. Please enter a valid number.");
        return;
    }

    println!("Temperature in Fahrenheit: {}", fahrenheit);
}
