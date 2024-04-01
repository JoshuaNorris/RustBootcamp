// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

fn main() {
	let box1 = BoxDetails::create_box(3,5, BoxColor::Red);
	let box2 = BoxDetails::create_box(3,5, BoxColor::Brown);
	let box3 = BoxDetails::create_box(3,5, BoxColor::Blue);
	box1.print_box();
	box2.print_box();
	box3.print_box();
}

#[derive(Debug)]
enum BoxColor {
	Red,
	Brown,
	Blue,
}

struct BoxDetails {
	dimensions: i32,
	weight: i32,
	color: BoxColor,
}


impl BoxDetails {
	fn create_box (d: i32, w: i32, c: BoxColor) -> Self {
		return Self {
			dimensions: d,
			weight: w,
			color: c,
		};
	}

	fn print_box(&self) {
		println!("The dimensions are {:?}, the weight is {:?}, and the color is {:?}", self.dimensions, self.weight, self.color);
	}
}





