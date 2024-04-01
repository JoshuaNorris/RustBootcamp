// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// ** Use a struct for a persons age, name, and favorite color
// ** The color and name should be stored as a String
// ** Create and store at least 3 people in a vector
// ** Iterate through the vector using a for..in loop
// ** Use an if expression to determine which person's info should be printed
// ** The name and colors should be printed using a function

fn main() {
	let p1 = Person::new(12, String::from("Joe"), String::from("Blue"));
	let p2 = Person::new(15, String::from("Bob"), String::from("Red"));
	let p3 = Person::new(18, String::from("Fred"), String::from("Yellow"));
	let people = vec![p1, p2, p3];

	for person in people {
		if person.age > 12 {
			person.print();
		}
	}

}

struct Person {
	age: u8,
	name: String,
	favorite_color: String,
}

impl Person {
	fn new (a:u8, n:String, c:String) -> Self {
		return Self {
			age: a,
			name: n,
			favorite_color: c,
		};
	}

	fn print (&self) {
		println!("age: {:?}", self.age);
		println!("name: {:?}", self.name);
		println!("favorite_color: {:?}", self.favorite_color);
	}
}