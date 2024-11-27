use std::io;

pub fn run() {

    loop {
        println!("Enter the first number:");
        let num1 = get_input();

        println!("Enter the operation you want to perform: +, -, *, /  -> ");
        let operation = get_operator();

        println!("Enter second number:");
        let num2 = get_input();

        let answer = match operation {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => {
				
                if num2 == 0.0 {
                    println!("Error: Cannot divide by zero.");
                    continue;
                } 
				else {
                    num1 / num2
                }
            }
            _ => {
                println!("Error: Invalid operation");
                continue;
            }
        };

        println!("Answer: {}", answer);

        println!("Do you want to perform another operation? (y/n):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid input");
        
        if input.trim().to_uppercase() != "Y" {
            break;
        }
    }
}

fn get_input() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid input");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input, please enter a valid number.");
            }
        }
    }
}

fn get_operator() -> char {
    loop {
        let mut operation = String::new();
        io::stdin()
            .read_line(&mut operation)
            .expect("Invalid input");
        let operation = operation.trim();
		
        if operation == "+" || operation == "-" || operation == "*" || operation == "/" {
            return operation.chars().next().unwrap();
        } 
		else {
            println!("Invalid operation, only +, -, *, / are valid.");
        }
    }
}
