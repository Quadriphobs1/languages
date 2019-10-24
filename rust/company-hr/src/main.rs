extern crate company_hr;

use company_hr::action::{ask_action, perform_action, print_actions};
use company_hr::department::*;
use company_hr::employee::*;

fn main() {
    let mut employees = Employees::new();
    let mut departments = Departments::new();
    println!("Welcome to Company HR. \nYou can manage employee with this program");
    println!("_____________________________________________");
    print_actions();
    loop {
        let action: i32 = ask_action();
        perform_action(&mut employees, &mut departments, action);
        // print!("{}[2J", 27 as char); // Clear the terminal when done with the operation before the next operation
    }
}
