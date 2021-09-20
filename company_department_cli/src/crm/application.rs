use std::fmt;
use std::io;

use super::database;
use super::input;

pub struct Application {
    database: database::Database,
}

impl Application {
    pub fn new() -> Self {
        Application {
            database: database::Database::new()
        }
    }

    pub fn run(&mut self) {
        loop {
            // print CLI title
            println!("Company Department CLI");
            println!("====================");

            // handle main menu
            let main_menu_choice = match self.main_menu() {
                None => continue,
                Some(main_menu_choice) => main_menu_choice,
            };
            match main_menu_choice {
                MainMenuChoice::AddEmployee => self.add_employee_menu(),
                MainMenuChoice::ListDepartmentEmployees => self.list_department_employees_menu(),
                MainMenuChoice::ListAllEmployees => self.list_all_employees_menu(),
                MainMenuChoice::Quit => return,
            }
            println!();
        }
    }

    fn main_menu(&self) -> Option<MainMenuChoice> {
        loop {
            // print main menu
            println!("Choose an option:");
            let main_menu_choices: &[MainMenuChoice; 4] = &[
                MainMenuChoice::AddEmployee,
                MainMenuChoice::ListDepartmentEmployees,
                MainMenuChoice::ListAllEmployees,
                MainMenuChoice::Quit,
            ];
            for (index, main_menu_choice) in main_menu_choices.iter().enumerate() {
                println!("{}) {}", index + 1, main_menu_choice);
            }

            // read-in choice
            let mut choice = String::new();
            match io::stdin().read_line(&mut choice) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error parsing input ({})\n", e);
                    return None;
                }
            }

            // convert to integer
            let choice: u32 = match choice.trim().parse() {
                Ok(num) => num,
                Err(e) => {
                    println!("Input was not an Integer ({})\n", e);
                    return None;
                }
            };

            // select appropriate main menu option
            match choice {
                1 => return Some(MainMenuChoice::AddEmployee),
                2 => return Some(MainMenuChoice::ListDepartmentEmployees),
                3 => return Some(MainMenuChoice::ListAllEmployees),
                4 => return Some(MainMenuChoice::Quit),
                _ => {
                    println!("Invalid choice\n");
                    continue;
                }
            }
        }
    }

    fn add_employee_menu(&mut self) {
        let department_name = input::get_department_name();
        let employee_name = input::get_employee_name();

        self.database.add_employee(&department_name, &employee_name);
    }

    fn list_department_employees_menu(&self) {
        let department_name = input::get_department_name();

        let employees = match self.database.get_department_employees(&department_name) {
            None => {
                println!("Department, {}, doesn't exist", department_name);
                return
            },
            Some(employees) => employees
        };

        println!("All employees in the {} department:", department_name);
        for employee_name in employees {
            println!("{}", employee_name);
        }
    }

    fn list_all_employees_menu(&self) {
        println!("List all employees, sorted by department\n");

        let company = self.database.get_all_employees();

        if company.len() == 0 {
            println!("Company has no employees");
            return;
        }

        for (department, employees) in company {
            println!("{} Employees:", department);

            for employee in employees {
                println!("{}", employee);
            }
        }
    }
}

pub enum MainMenuChoice {
    AddEmployee,
    ListDepartmentEmployees,
    ListAllEmployees,
    Quit,
}

impl fmt::Display for MainMenuChoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MainMenuChoice::AddEmployee => write!(f, "Add Employee to Department"),
            MainMenuChoice::ListDepartmentEmployees => write!(f, "List Department Employees"),
            MainMenuChoice::ListAllEmployees => write!(f, "List All Employees"),
            MainMenuChoice::Quit => write!(f, "Quit"),
        }
    }
}
