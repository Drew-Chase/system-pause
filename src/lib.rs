#[macro_export]
macro_rules! pause {
    () => {
	    println!("Press Enter to continue...");
	    let mut input = String::new();
	    std::io::stdin().read_line(&mut input).unwrap();
    };
}