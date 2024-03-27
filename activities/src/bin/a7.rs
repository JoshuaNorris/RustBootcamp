// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

fn main() {
	which_color(Color::Red);
	which_color(Color::Blue);
	which_color(Color::Green);
	which_color(Color::Yellow);
}

fn which_color(color:Color) {
	match color {
		Color::Red => println!("Red"),
		Color::Blue => println!("Blue"),
		Color::Green => println!("Green"),
		Color::Yellow => println!("Yellow"),
	}
}

enum Color {
	Red,
	Blue,
	Green,
	Yellow
}
