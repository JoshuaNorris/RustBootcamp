// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
//  Use an enum to represent all types of employees
//  Use a struct to store the employee type and whether they are
//   still employed
//  Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

fn main() -> Result<(), String> {

	let emp1 = Employee { employment: true, department: Department::Maintenance };
	let emp2 = Employee { employment: false, department: Department::Marketing };
	let emp3 = Employee { employment: false, department: Department::Manager };
	let emp4 = Employee { employment: true, department: Department::LineSupervisor };
	let emp5 = Employee { employment: true, department: Department::Kitchen };
	let emp6 = Employee { employment: true, department: Department::Assembly };

	println!("{:?}", can_access(emp1)?);
	println!("{:?}", can_access(emp2)?);
	println!("{:?}", can_access(emp3)?);
	println!("{:?}", can_access(emp4)?);
	println!("{:?}", can_access(emp5)?);
	println!("{:?}", can_access(emp6)?);
	Ok(())
}

#[derive(PartialEq)]
enum Department {
	Maintenance,
	Marketing,
	Manager,
	LineSupervisor,
	Kitchen,
	Assembly,
}

struct Employee {
	employment: bool,
	department: Department,
}

fn can_access(emp: Employee) -> Result<bool, String> {
	if emp.department == Department::Maintenance || emp.department == Department::Maintenance || emp.department == Department::Maintenance {
		if emp.employment {
			return Ok(true);
		}
	}
	return Err(String::from("This employee cannot access the building."));
}







