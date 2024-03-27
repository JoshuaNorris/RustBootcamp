// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
	torf(true);
	torf(false);

}


fn torf (input: bool) {
	match input {
		true => println!("its true!"),
		false => println!("its false!"),
	}
}