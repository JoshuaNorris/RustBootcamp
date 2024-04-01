// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

fn main() {
	let bananas = GroceryItem {
		id : 9395,
		quantity : 4,
	};
	get_quantity(&bananas);
	get_id(&bananas);
}


struct GroceryItem {
	id : i32,
	quantity : i32,
}

fn get_quantity (item : &GroceryItem) {
	print!("{:?}", item.quantity);
}

fn get_id (item : &GroceryItem) {
	print!("{:?}", item.id);
}