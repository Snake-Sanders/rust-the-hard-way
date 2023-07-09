use std::io;

fn main() {
    let mut user_name = String::new();
    println!("Enter your name:");
    io::stdin()
        .read_line(&mut user_name)
        .expect("Failed to read user_name");
    println!("Hello, {}!", user_name.trim());
}
