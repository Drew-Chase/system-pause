use system_pause::pause_with_message;

fn main() {
    println!("Hello, world!");
	// Pauses the program execution and displays a custom message to the user.
	// The program will wait until the user presses the Enter key.
    pause_with_message!("Please wait a moment...");
    println!("Goodbye, world!");
}
