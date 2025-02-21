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
        {
            pause!("Press Enter to continue..."); // Calls the `pause_with_message!` macro with a default message.
        }
    };

    ($msg:expr) => {
        {
            println!($msg); // Prints the custom pause message.
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap(); // Waits for user input (Enter key) to continue.
        }
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
#[deprecated(since = "0.1.1", note = "Use `pause!` instead.")]
#[macro_export]
macro_rules! pause_with_message {
    ($msg:expr) => {
        {
            println!($msg); // Prints the custom pause message.
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap(); // Waits for user input (Enter key) to continue.
        }
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
    // Pattern for just seconds parameter
    ($seconds:expr) => {
        pause_for_time!($seconds, "Wait {}s to continue...") // Default message
    };

    // Pattern for both seconds and message parameters
    ($seconds:expr, $message:expr) => {{
        use std::io::Write;
        let message_length: usize = $message.len();
        let mut time_remaining = $seconds;
        let mut clear_line = false;
        while time_remaining >= 0 {
            print!(
                "\r{}\r{}",
                " ".repeat(message_length + 5),
                format!($message, time_remaining)
            );
            std::io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
            time_remaining -= 1;
        }

        let system_pause_options = std::env::var("SYSTEM_PAUSE");
        if let Ok(options_list) = system_pause_options {
            let options = options_list.split(',');
            for option in options {
                let key_value: Vec<&str> = option.split('=').collect();
                let key = key_value[0];
                let value = key_value[1];
                if key == "CLEAR_TIMER_LINE" && value == "true" {
                    clear_line = true;
                }
            }
        }
        if clear_line {
            print!("\r{}\r", " ".repeat(message_length + 5));
            std::io::stdout().flush().unwrap();
        } else {
            println!();
        }
    }};
}
