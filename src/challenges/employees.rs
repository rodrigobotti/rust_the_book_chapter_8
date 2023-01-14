use regex::Regex;

#[derive(Debug)]
pub struct Employee {
    pub name: String,
    pub department: String,
}

#[derive(Debug)]
pub enum EmployeeCommand {
    Add(Employee),
    List(String),
    ListAll,
}

pub fn parse_command(command: &str) -> Option<EmployeeCommand> {
    let add_command_regex = Regex::new(r"Add (\w+) to (\w+)").unwrap();
    let list_command_regex = Regex::new(r"List employees from (\w+)").unwrap();
    const LIST_ALL_COMMAND: &str = "List all employees";

    if command == LIST_ALL_COMMAND {
        return Some(EmployeeCommand::ListAll);
    }

    if let Some(captures) = add_command_regex.captures(&command) {
        let name_option = captures.get(1);
        let dept = captures.get(2);

        match (name_option, dept) {
            (Some(name), Some(dept)) => Some(EmployeeCommand::Add(Employee {
                name: String::from(name.as_str()),
                department: String::from(dept.as_str()),
            })),
            _ => None,
        }
    } else if let Some(captures) = list_command_regex.captures(&command) {
        let dept_option = captures.get(1);
        dept_option
            .map(|dept| String::from(dept.as_str()))
            .map(|dept| EmployeeCommand::List(dept))
    } else {
        None
    }
}
