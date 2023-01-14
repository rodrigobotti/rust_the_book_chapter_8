use regex::Regex;
use std::collections::HashMap;
use std::io;

fn main() {
    median_program();

    separator();

    mode_program();

    separator();

    pig_latin_program();

    separator();

    employee_program();
}

fn separator() {
    println!("{}", "-".repeat(100));
}

fn median_program() {
    let xs = [1, 2, 3, 4, 5, 6, 7];
    let xs_median = median(&xs);
    println!("{:?} has {} as median", xs, xs_median);
}

fn mode_program() {
    let xs = [1, 2, 1, 1, 3, 3, 4, 5];
    let xs_mode = mode(&xs);
    println!("{:?} has {} as mode", xs, xs_mode);
}

fn pig_latin_program() {
    let with_consonant = String::from("first");
    println!(
        "{} = {} in pig latin",
        with_consonant,
        pig_latin(&with_consonant)
    );

    let with_vowel = String::from("apple");
    println!("{} = {} in pig latin", with_vowel, pig_latin(&with_vowel));

    let non_ascii = String::from("こんにちは");
    println!("{} = {} in pig latin", non_ascii, pig_latin(&non_ascii));
}

fn is_even(x: &usize) -> bool {
    x % 2 == 0
}

fn median(xs: &[i32]) -> f64 {
    let length = xs.len();
    assert!(length > 0, "xs must have elements");

    let mut to_sort = xs.to_vec();

    to_sort.sort();

    let middle_idx = (length as f64 / 2.0).floor() as usize;

    if is_even(&length) {
        let mid_1 = to_sort.get(middle_idx - 1).cloned().unwrap();
        let mid_2 = to_sort.get(middle_idx).cloned().unwrap();
        return f64::from(mid_1 + mid_2) / 2.0;
    }

    let median = to_sort.get(middle_idx).cloned().unwrap();
    f64::from(median)
}

fn mode(xs: &[i32]) -> i32 {
    assert!(xs.len() > 0, "xs must have elements");

    let mut frequencies: HashMap<i32, u32> = HashMap::new();

    for &x in xs {
        let frequency = frequencies.entry(x).or_insert(0);
        *frequency += 1;
    }

    let mut mode: i32 = 0;
    let mut highest_frequency: u32 = 0;

    for (num, frequency) in frequencies {
        if frequency > highest_frequency {
            highest_frequency = frequency;
            mode = num;
        }
    }

    mode
}

const CHAR_CONSONANTS: [char; 42] = [
    'B', 'C', 'D', 'F', 'G', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'S', 'T', 'V', 'X', 'Z', 'H', 'R',
    'W', 'Y', 'b', 'c', 'd', 'f', 'g', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 's', 't', 'v', 'x', 'z',
    'h', 'r', 'w', 'y',
];

const CHAR_VOWELS: [char; 10] = ['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];

fn is_consonant(c: &char) -> bool {
    CHAR_CONSONANTS.contains(c)
}
fn is_vowel(c: &char) -> bool {
    CHAR_VOWELS.contains(c)
}

fn pig_latin(word: &String) -> String {
    let mut new_word = String::new();
    let mut is_first_char = true;
    let mut has_first_consonant = false;
    let mut first_consonant = String::new();
    let mut has_first_vowel = false;
    let mut chars = word.chars().peekable();

    loop {
        let next_element = chars.next();
        let is_last_char = match chars.peek() {
            Some(_) => false,
            None => true,
        };
        match next_element {
            Some(c) => {
                if is_first_char {
                    is_first_char = false;
                    if is_consonant(&c) {
                        has_first_consonant = true;
                        first_consonant.push(c);
                        continue;
                    }
                    if is_vowel(&c) {
                        has_first_vowel = true;
                        new_word.push(c);
                        continue;
                    }
                    new_word.push(c);
                    continue;
                }
                new_word.push(c);
                if is_last_char && has_first_vowel {
                    new_word.push_str("-hay");
                    continue;
                }
                if is_last_char && has_first_consonant {
                    new_word.push_str(&format!("-{}ay", first_consonant));
                }
            }
            None => break new_word,
        }
    }
}

#[derive(Debug)]
struct Employee {
    name: String,
    department: String,
}

#[derive(Debug)]
enum EmployeeCommand {
    Add(Employee),
    List(String),
    ListAll,
}

fn parse_command(command: &str) -> Option<EmployeeCommand> {
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

fn ask_validate_command() -> EmployeeCommand {
    loop {
        println!("Type in an employee command");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("** Failed to read from stdin **");

        match parse_command(&input.trim()) {
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
        "{}",
        "
    Welcome to the employee management program    
    valid employee commands:
    - Add {Employee} to {Department}
    - List employees from {Department}
    - List all employees
    "
    );

    loop {
        let employee_command = ask_validate_command();

        match employee_command {
            EmployeeCommand::Add(Employee { name, department }) => {
                println!(">> Adding {} to {}", name, department);
                let employee_list = company.entry(department).or_insert(vec![]);
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
            EmployeeCommand::ListAll => println!(">> {:?}", company),
        }
    }
}
