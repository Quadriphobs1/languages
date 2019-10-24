use crate::department::*;
use crate::title_case;
use std::fmt::{Display, Formatter, Result};
use std::io::stdin;

struct Employee {
  name: String,
  department: String,
}

impl Display for Employee {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(
      f,
      "Name {} - Department {}",
      title_case(&self.name),
      title_case(&self.department)
    )
  }
}

pub struct Employees {
  data: Vec<Employee>,
}

impl Employees {
  pub fn new() -> Employees {
    Employees { data: Vec::new() }
  }

  fn list(&self) {
    for employee in &self.data {
      println!("{}", &employee);
    }
  }
  fn list_department(&mut self, dpt: &String) {
    for employee in &self.find_in_dept(dpt.clone()) {
      println!("{}", &employee);
    }
  }

  fn add(&mut self, name: String, department: String) {
    let employee = Employee { name, department };
    self.data.push(employee);
  }

  fn find_in_dept(&mut self, dpt: String) -> Vec<Employee> {
    self
      .data
      .drain(..)
      .filter(|emp| emp.department == dpt)
      .collect::<Vec<Employee>>()
  }

  fn find_index(&self, name: String) -> Option<usize> {
    match &self.data.iter().position(|e| *e.name == name) {
      Some(id) => Some(*id),
      None => None,
    }
  }

  fn remove(&mut self, idx: usize) {
    self.data.remove(idx);
  }
}

pub fn add_employee(employees: &mut Employees, departments: &mut Departments) {
  println!("\n");
  println!("Enter employee's name");
  let mut name = String::new();
  stdin()
    .read_line(&mut name)
    .expect("Enter a valid employee name");
  let name = name.trim().to_string();
  println!("Enter employee's department");
  let mut dpt = String::new();
  stdin()
    .read_line(&mut dpt)
    .expect("Enter a valid department name");
  let dpt = dpt.trim().to_string();
  &employees.add(name.to_lowercase().clone(), dpt.to_lowercase().clone());
  &departments.inc_department_staff(dpt.to_lowercase().clone());

  println!("Added {} to {} \n\n", name, dpt);
}

pub fn employees_in_department(employees: &mut Employees, departments: &mut Departments) {
  println!("Enter the department name?");

  let mut dpt = String::new();

  stdin()
    .read_line(&mut dpt)
    .expect("Enter a valid department name");
  let dpt = dpt.trim().to_string();
  if let Some(_dpt_staff_no) = &departments.find(dpt.clone()) {
    &employees.list_department(&dpt);
  } else {
    println!("The provided department does not exist")
  }
}

pub fn list_employees(employees: &Employees) {
  if employees.data.len() == 0 {
    println!("You currently have no employee in record \n")
  } else {
    println!("Employee's list");
    println!("____________");
    employees.list();
  }
}

pub fn remove_employee(employees: &mut Employees) {
  println!("Enter the name of the employee you want to remove");
  let mut name = String::new();
  stdin()
    .read_line(&mut name)
    .expect("Enter the name of the employee you want to remove");
  let name = name.trim().to_string();

  if let Some(id) = employees.find_index(name.to_lowercase().clone()) {
    employees.remove(id);
  } else {
    println!("There is no employee record with the name: {} \n", name);
  }
}
