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
        let name = String::from(captures.get(1)?.as_str());
        let department = String::from(captures.get(2)?.as_str());

        Some(EmployeeCommand::Add(Employee { name, department }))
    } else if let Some(captures) = list_command_regex.captures(&command) {
        captures
            .get(1)
            .map(|dept| String::from(dept.as_str()))
            .map(|dept| EmployeeCommand::List(dept))
    } else {
        None
    }
}
