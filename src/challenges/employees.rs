use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Employee {
    pub name: String,
    pub department: String,
}

const LIST_ALL_COMMAND: &str = "List all employees";
lazy_static! {
    static ref ADD_COMMAND_REGEX: Regex = Regex::new(r"Add (\w+) to (\w+)").unwrap();
    static ref LIST_COMMAND_REGEX: Regex = Regex::new(r"List employees from (\w+)").unwrap();
}

#[derive(Debug, PartialEq)]
pub enum EmployeeCommand {
    Add(Employee),
    List(String),
    ListAll,
}

impl EmployeeCommand {
    pub fn parse(command: &str) -> Option<EmployeeCommand> {
        if command == LIST_ALL_COMMAND {
            return Some(EmployeeCommand::ListAll);
        }

        if let Some(captures) = ADD_COMMAND_REGEX.captures(command) {
            let name = String::from(captures.get(1)?.as_str());
            let department = String::from(captures.get(2)?.as_str());

            Some(EmployeeCommand::Add(Employee { name, department }))
        } else if let Some(captures) = LIST_COMMAND_REGEX.captures(command) {
            captures
                .get(1)
                .map(|dept| String::from(dept.as_str()))
                .map(EmployeeCommand::List)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parses_add_command() {
        let name = "Botti".to_string();
        let department = "Engineering".to_string();
        let command_str = format!("Add {name} to {department}");
        let expected = EmployeeCommand::Add(Employee { name, department });

        let command = EmployeeCommand::parse(&command_str).unwrap();

        assert_eq!(command, expected);
    }

    #[test]
    fn test_parses_list_command() {
        let department = "Product".to_string();
        let command_str = format!("List employees from {department}");
        let expected = EmployeeCommand::List(department);

        let command = EmployeeCommand::parse(&command_str).unwrap();

        assert_eq!(command, expected);
    }

    #[test]
    fn test_parses_list_all_command() {
        let command_str = "List all employees";

        let command = EmployeeCommand::parse(command_str).unwrap();

        assert_eq!(command, EmployeeCommand::ListAll);
    }

    #[test]
    fn test_returns_whrn_parsing_fails() {
        let command_str = "not a valid command";

        let result = EmployeeCommand::parse(command_str);

        assert!(result.is_none());
    }
}
