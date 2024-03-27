// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:
// * Use a variable set to either true or false
// * Use an if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {
	let bool_flag:bool = false;
	greeting(bool_flag);
}

fn greeting(bool_flag: bool) {
	if bool_flag {
		println!("hello");
	} else {
		println!("goodbye");
	}
}
