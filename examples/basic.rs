use system_pause::{pause};

fn main() {
    println!("Hello, world!");
    // Pauses the program execution and prompts the user with a default message.
    // The program will wait until the user presses the Enter key.
    pause!();
    println!("Goodbye, world!");
}
