// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

fn main() {
	which_drink(Drink{
		flavor: DrinkFlavor::Cherry,
		ounces: 8
	});

	which_drink(Drink{
		flavor: DrinkFlavor::Ginger,
		ounces: 32
	});

	which_drink(Drink{
		flavor: DrinkFlavor::Blueberry,
		ounces: 16
	});

	which_drink(Drink{
		flavor: DrinkFlavor::Lime,
		ounces: 4
	});
}

fn which_drink(drink:Drink) {
	match drink.flavor {
		DrinkFlavor::Cherry => println!("This is a cherry drink and is {:?} ounces.", drink.ounces),
		DrinkFlavor::Blueberry => println!("This is a blueberry drink and is {:?} ounces.", drink.ounces),
		DrinkFlavor::Lime => println!("This is a lime drink and is {:?} ounces.", drink.ounces),
		DrinkFlavor::Ginger => println!("This is a ginger drink and is {:?} ounces.", drink.ounces),
	}
}


enum DrinkFlavor {
	Cherry,
	Blueberry,
	Lime,
	Ginger
}

struct Drink {
	flavor: DrinkFlavor,
	ounces: u8,
}