/// Macro: `pause!`
///
/// Pauses the program execution and prompts the user with a default message.
/// The program will wait until the user presses the Enter key.
///
/// Example:
/// ```no-rust
/// pause!();
/// ```
#[macro_export]
macro_rules! pause {
    () => {
        use system_pause::pause_with_message;
        pause_with_message!("Press Enter to continue..."); // Calls the `pause_with_message!` macro with a default message.
    };
}

/// Macro: `pause_with_message!`
///
/// Pauses the program execution and displays a custom message to the user.
/// The program will wait until the user presses the Enter key.
///
/// # Arguments
/// * `$msg`: The custom message to display before pausing.
///
/// Example:
/// ```no-rust
/// pause_with_message!("Custom pause message...");
/// ```
#[macro_export]
macro_rules! pause_with_message {
    ($msg:expr) => {
        println!($msg); // Prints the custom pause message.
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap(); // Waits for user input (Enter key) to continue.
    };
}

/// Macro: `pause_for_time!`
///
/// Pauses the program execution for a specified number of seconds and displays a countdown.
/// During the pause, the remaining time is updated on the same console line.
///
/// # Arguments
/// * `$seconds`: The total number of seconds to wait.
///
/// Example:
/// ```no-rust
/// pause_for_time!(5); // Pauses for 5 seconds and shows a countdown.
/// ```
#[macro_export]
macro_rules! pause_for_time {
    ($seconds:expr) => {
        use std::io::Write; // Required for flushing the output stream.
        let mut time_remaining = $seconds; // Initializes the countdown timer.
        while time_remaining > 0 { // Continues until the timer reaches 0.
            print!(
                "\r{}\rWait {}s to continue...",
                " ".repeat(27), // Clears the previous line by overwriting it with spaces.
                time_remaining // Displays the remaining time.
            );
            std::io::stdout().flush().unwrap(); // Ensures the output is displayed immediately.
            std::thread::sleep(std::time::Duration::from_secs(1)); // Waits for 1 second.
            time_remaining -= 1; // Decrements the countdown.
        }
        println!(); // Adds a new line after the countdown is complete.
    };
}