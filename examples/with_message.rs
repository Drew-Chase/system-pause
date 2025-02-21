use system_pause::pause;

fn main() {
    println!("Hello, world!");
    // Pauses the program execution and displays a custom message to the user.
    // The program will wait until the user presses the Enter key.
    pause!("Please wait a moment...");
    println!("Goodbye, world!");
}
