// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * * Use a struct containing the student's name and locker assignment
// * * The locker assignment should use an Option<i32>

fn main() {
	let student1 = Student {
		name: String::from("Sue"),
		locker: Some(4)
	};
	let student2 = Student {
		name: String::from("Mary"),
		locker: None
	};
	print_student(student1);
	print_student(student2);
}

fn print_student(stu: Student) {
	match stu {
		Student {name, locker: Some(locker)} => println!("My name is {:?} and my locker number is {:?}.", name, locker ),
		Student {name, locker: None} => println!("My name is {:?} and I don't have a locker", name ),
	};
}

struct Student {
	name: String,
	locker: Option<i32>,
}