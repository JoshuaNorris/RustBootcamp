// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

fn main() {
	let tickets: Vec<Ticket> = vec![Ticket::Backstage(50, String::from("John")), 
									Ticket::Vip(25, String::from("Bob")), 
									Ticket::Standard(10)];
	print_tickets(tickets);

}


enum Ticket {
	Backstage(i32, String),
	Vip(i32, String),
	Standard(i32)
}

fn print_tickets (tics:Vec<Ticket>) {

	for tic in tics {
		match tic {
			Ticket::Backstage(price, name) => println!("Backstage : {:?}. They paid ${:?}", name, price),
			Ticket::Vip(price, name) => println!("Vip : {:?}. They paid ${:?}", name, price),
			Ticket::Standard(price) => println!("Standard. They paid ${:?}", price),
		}
	}
}