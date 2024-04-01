// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn main() {
	big_or_small(50);
	big_or_small(150);
}


fn big_or_small(num : i32) {
	let result = match num {
		n if n > 100 => "its big",
		n if n <= 100 => "its small",
		_ => "math doesn't make sense",
	};
	println!("{:?}", result);
}