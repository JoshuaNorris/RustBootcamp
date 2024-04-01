// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
	let nums = vec![10, 20, 30, 40];
	find_thirty(nums);
}


fn find_thirty(nums: Vec<i32>) {
	println!("{:?}", &nums.len());
	for num in nums {
		if num == 30 {
			println!("thirty");
		} else {
			println!("{num}");
		}
	}
}