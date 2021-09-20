use std::collections::HashMap;

pub struct Database {
	// Department Name -> Vec<Employee Name>
	db: HashMap<String, Vec<String>>
}

impl Database {
    pub fn new() -> Self {
        Database {
            db: HashMap::new()
        }
    }

	pub fn add_employee(&mut self, department_name: &String, employee_name: &String) {
        let employees = self.db
            .entry(department_name.to_string())
            .or_insert(Vec::new());
        employees.push(employee_name.to_string());
	}

	pub fn get_department_employees(&self, department_name: &String) -> Option<Vec<String>> {
        // get cloned employees list (if it exists)
        let mut employees = match self.db.get(department_name) {
            None => return None,
            Some(employees) => employees
        }.clone();

        // sort it alphabetically
        employees.sort();
        Some(employees)
	}

	pub fn get_all_employees(&self) -> &HashMap<String, Vec<String>> {
        &self.db
	}
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Database {
            db: self.db.clone()
        }
    }
}
