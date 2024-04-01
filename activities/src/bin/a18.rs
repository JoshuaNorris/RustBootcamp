// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
//  Use a struct to store at least the age of a customer
//  Use a function to determine if a customer can make a restricted purchase
//  Return a result from the function
// The Err variant should detail the reason why they cannot make a purchase

fn main() {
	let cust1 = Customer {age : 20};
	let cust2 = Customer {age : 22};
	println!("{:?}", is_customer_old_enough(cust1));
	println!("{:?}", is_customer_old_enough(cust2));
}

struct Customer {
	age: i32
}


fn is_customer_old_enough(cust: Customer) -> Result <bool, String> {
	if cust.age >= 21 {
		return Ok(true);
	}
	else {
		return Err(String::from("Customer is not old enough"));
	}
}