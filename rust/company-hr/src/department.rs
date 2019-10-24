use crate::title_case;
use std::collections::HashMap;
use std::io::stdin;

pub struct Departments {
  data: HashMap<String, i32>,
}

impl Departments {
  pub fn new() -> Departments {
    Departments {
      data: HashMap::new(),
    }
  }

  pub fn add(&mut self, dept: String) {
    self.data.insert(dept, 0);
  }

  pub fn find(&mut self, dpt: String) -> Option<&i32> {
    self.data.get(&dpt)
  }

  pub fn inc_department_staff(&mut self, dep: String) {
    *self.data.entry(dep).or_insert(0) += 1;
  }

  pub fn dec_department_staff(&mut self, dep: String) {
    *self.data.entry(dep).or_insert(0) -= 1;
  }

  pub fn list(&self) {
    for (dpt, emp) in &self.data {
      println!("Name {}({} employees)", title_case(dpt), emp);
    }
  }
}

pub fn add_department(departments: &mut Departments) {
  println!("Enter the department name:");
  let mut dept = String::new();
  stdin()
    .read_line(&mut dept)
    .expect("Enter valid department name:");
  let dept = dept.trim().to_string();
  &departments.add(dept.to_lowercase().clone());
  println!("{} has been added to the departments \n", dept);
}

pub fn list_department(departments: &Departments) {
  departments.list()
}
