// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn main() {
	which_num(1);
	which_num(2);
	which_num(3);
	which_num(4);

}

fn which_num(num:i32) {
	match num {
		1 => println!("one"),
		2 => println!("two"),
		3 => println!("three"),
		_ => println!("greater than three"),
	}
}
