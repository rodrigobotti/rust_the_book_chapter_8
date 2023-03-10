use std::collections::HashMap;
use std::io;

use chapter_eight::challenges::employees::{Employee, EmployeeCommand};
use chapter_eight::challenges::math;
use chapter_eight::challenges::pig_latin;
use chapter_eight::utils;

fn main() {
    median_program();

    utils::print_separator();

    mode_program();

    utils::print_separator();

    pig_latin_program();

    utils::print_separator();

    employee_program();
}

fn median_program() {
    let xs = [1, 2, 3, 4, 5, 6, 7];
    let xs_median = math::median(&xs);
    println!("{xs:?} has {xs_median} as median");
}

fn mode_program() {
    let xs = [1, 2, 1, 1, 3, 3, 4, 5];
    let xs_mode = math::mode(&xs);
    println!("{xs:?} has {xs_mode} as mode");
}

fn pig_latin_program() {
    let with_consonant = String::from("first");
    println!(
        "{} = {} in pig latin",
        with_consonant,
        pig_latin::to_pig_latin(&with_consonant)
    );

    let with_vowel = String::from("apple");
    println!(
        "{} = {} in pig latin",
        with_vowel,
        pig_latin::to_pig_latin(&with_vowel)
    );

    let non_ascii = String::from("こんにちは");
    println!(
        "{} = {} in pig latin",
        non_ascii,
        pig_latin::to_pig_latin(&non_ascii)
    );
}

fn ask_validate_command() -> EmployeeCommand {
    loop {
        println!("Type in an employee command");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("** Failed to read from stdin **");

        match EmployeeCommand::parse(input.trim()) {
            Some(command) => break command,
            None => {
                println!("** Please type in a valid command text **");
                continue;
            }
        }
    }
}

fn employee_program() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!(
        "
    Welcome to the employee management program    
    valid employee commands:
    - Add <Employee> to <Department>
    - List employees from <Department>
    - List all employees
    "
    );

    loop {
        let employee_command = ask_validate_command();

        match employee_command {
            EmployeeCommand::Add(Employee { name, department }) => {
                println!(">> Adding {name} to {department}");
                let employee_list = company.entry(department).or_default();
                employee_list.push(name);
            }
            EmployeeCommand::List(dept) => match company.get(&dept) {
                Some(employees) => {
                    let mut sorted = employees.to_vec();
                    sorted.sort();
                    println!(">> Employees in {dept}: {:}", sorted.join(", "));
                }
                None => println!(">> No employees in this department"),
            },
            EmployeeCommand::ListAll => println!(">> {company:?}"),
        }
    }
}
