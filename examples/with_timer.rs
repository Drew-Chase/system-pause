use system_pause::pause_for_time;

fn main() {
	println!("Hello, world!");
	// Pauses the program execution for a specified number of seconds and displays a countdown.
	// During the pause, the remaining time is updated on the same console line.
	// Note: This is blocks the current thread, use accordingly.
	pause_for_time!(5);
	std::env::set_var("SYSTEM_PAUSE", "CLEAR_TIMER_LINE=false");
	pause_for_time!(5, "This line should be cleared after {} seconds.");
	println!("Goodbye, world!");
}
