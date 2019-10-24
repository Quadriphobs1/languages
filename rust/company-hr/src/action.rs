use crate::department::*;
use crate::employee::*;
use std::io;
use std::num::ParseIntError;
use std::process;

pub fn ask_action() -> i32 {
  check_action()
}

pub fn print_actions() {
  println!(
    "
  Enter '1' to add an employee to a department. \n
  Enter '2' to list employee within a department. \n
  Enter '3' to list all company employee. \n
  Enter '4' to add a department. \n
  Enter '5' to list all department. \n
  Enter '6' Remove an employee. \n
  Enter '7' to exit.
  "
  );
}

fn check_action() -> i32 {
  let action = get_action();

  match action {
    Ok(i) => i,
    Err(_) => {
      println!("Please enter a number corresponding to the options above");
      check_action()
    }
  }
}

fn get_action() -> Result<i32, ParseIntError> {
  let mut action = String::new();
  io::stdin()
    .read_line(&mut action)
    .expect("Failed to get line");
  action.trim().parse::<i32>()
}

pub fn perform_action(employees: &mut Employees, departments: &mut Departments, action: i32) {
  match action {
    1 => add_employee(employees, departments),
    2 => employees_in_department(employees, departments),
    3 => list_employees(employees),
    4 => add_department(departments),
    5 => list_department(departments),
    6 => remove_employee(employees),
    7 => process::exit(0x0100),
    _ => (),
  }
}
