#![allow(unused)]
use std::io;

// enum Operation with variants Add, Subtract, Multiply, and Divide.
// Each variant holds two f64 values
#[derive(Debug)]
enum Operation {
    Add { x: f64, y: f64 },
    Substract { x: f64, y: f64 },
    Multiply { x: f64, y: f64 },
    Divide { x: f64, y: f64 },
}

impl Operation {
    // function calculate() takes an Operation enum as an argument and returns an f64 result
    fn calculate(&self) -> f64 {
        match self {
            Operation::Add { x, y } => x + y,
            Operation::Substract { x, y } => x - y,
            Operation::Multiply { x, y } => x * y,
            Operation::Divide { x, y } => x / y,
        }
    }
}

fn main() {
    let mut first_number = String::new();
    let mut second_number = String::new();
    let mut op_value = String::new();
    let mut op: Operation;

    loop {
        println!("Please input first number: ");

        first_number = "".to_string();

        io::stdin()
            .read_line(&mut first_number)
            .expect("Failed to read line");

        match first_number.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                println!("The number should be f64(real number)");
                continue;
            }
        };
        break;
    }

    loop {
        println!("Please input second number: ");

        second_number = "".to_string();

        io::stdin()
            .read_line(&mut second_number)
            .expect("Failed to read line");

        match second_number.trim().parse::<f64>() {
            Ok(num) => {
                if num == 0.0 {
                    println!("Second number shouldn't be equal to zero");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("The number should be f64(real number)");
                continue;
            }
        };
        break;
    }

    loop {
        println!("Please input one of the following operations: 'Add, Subtract, Multiply, Divide'");

        op_value = "".to_string();

        io::stdin()
            .read_line(&mut op_value)
            .expect("Failed to read line");

        match op_value.trim() {
            "Add" => {
                op = Operation::Add {
                    x: first_number.trim().parse::<f64>().unwrap(),
                    y: second_number.trim().parse::<f64>().unwrap(),
                };
            }
            "Subtract" => {
                op = Operation::Substract {
                    x: first_number.trim().parse::<f64>().unwrap(),
                    y: second_number.trim().parse::<f64>().unwrap(),
                };
            }
            "Multiply" => {
                op = Operation::Multiply {
                    x: first_number.trim().parse::<f64>().unwrap(),
                    y: second_number.trim().parse::<f64>().unwrap(),
                };
            }
            "Divide" => {
                op = Operation::Divide {
                    x: first_number.trim().parse::<f64>().unwrap(),
                    y: second_number.trim().parse::<f64>().unwrap(),
                };
            }
            _ => {
                println!(" should be one of the four operations");
                continue;
            }
        };
        break;
    }

    let op_res = op.calculate();
    println!("Operation result is: {:?}", op_res);
}
