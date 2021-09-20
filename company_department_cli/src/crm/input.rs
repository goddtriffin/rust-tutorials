use std::io;

pub fn get_employee_name() -> String {
    loop {
        println!("Enter Employee Name:");

        // read-in employee name
        let mut employee_name = String::new();
        match io::stdin().read_line(&mut employee_name) {
            Ok(_) => (),
            Err(e) => {
                println!("Error parsing input ({})\n", e);
                continue;
            }
        }

        // remove newline character
        employee_name.pop();

        // disallow empty string
        if employee_name.len() == 0 {
            println!("Cannot be empty string");
            continue;
        }

        return employee_name;
    }
}

pub fn get_department_name() -> String {
    loop {
        println!("Enter Department:");

        // read-in department
        let mut department = String::new();
        match io::stdin().read_line(&mut department) {
            Ok(_) => (),
            Err(e) => {
                println!("Error parsing input ({})", e);
                continue;
            }
        }

        // remove newline character
        department.pop();

        // disallow empty string
        if department.len() == 0 {
            println!("Cannot be empty string");
            continue;
        }

        return department;
    }
}
