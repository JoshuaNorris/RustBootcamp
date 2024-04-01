// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn main() {
	println!("{:?}", compare_to_five(6).1);
}

fn compare_to_five(y : i32) -> (i32, String) {
	let mut result = String::new();
	if 5 > y {
		result.push_str(">5");
		return (y, result);
	} else if 5 < y {
		result.push_str("<5");
		return (y, result);
	} else {
		result.push_str("=5");
		return (y, result);
	}
}
