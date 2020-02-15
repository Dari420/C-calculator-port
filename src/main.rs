use std::io::{stdin};
use std::time::Duration;
use std::thread::{sleep};
use std::process::exit;

enum Value {
    Float(f64),
}

fn main() {
    loop {
        calculator_choice();
        ask_again();
    }
}

fn parse_string(s: &str) -> Option<Value> {
    if let Ok(f) = s.parse() {  // inferred as isize from next line
        Some(Value::Float(f))
    } else {
        None
    }
}

fn calculator_choice() {
    loop {
        let mut cal_choice = String::new();
        println!("a) Addition");
        println!("b) Subtraction ");
        println!("c) Multiplication");
        println!("d) Division");
        println!("e) Square Roots");
        println!("f) Squares");
        stdin()
            .read_line(&mut cal_choice)
            .expect("invalid input! crashing");
        if calculations(cal_choice.to_owned()) {
            break;
        } else {
            ()
        }
    }
}

fn ask_again(){
    loop {
        let mut again_or_not = String::new();
        println! ("Go again? y/n");
        stdin()
            .read_line(&mut again_or_not)
            .expect("Invalid entry! crashing");
        match again_or_not.trim_end() {
            "y" => {
                break;
            },
            "n" => {
                println! ("Closing calculator");
                sleep(Duration::new(1, 0));
                exit(0);
            },
            _ => println! ("Invalid entry! Please choose y or n")
        }
    }
}

fn calculations(choice: String) -> bool{
    if choice.trim_end() == "a" || choice.trim_end() == "b" ||
       choice.trim_end() == "c" || choice.trim_end() == "d" ||
       choice.trim_end() == "e" || choice.trim_end() == "f" {
        ()
    }
    else {
        println! ("Invalid input! Please choose a, b, c, d, e, or f");
        return false;
    }
    'outer: loop {
        let mut user_input1 = String::new();
        println! ("Enter first number:");
        stdin()
            .read_line(&mut user_input1)
            .expect("Program error, crashing");
        let no_enter_input1: &str = &user_input1.replace("\n", "");
        match parse_string(&no_enter_input1) {
            Some(Value::Float(f)) => {
                match choice.trim_end() {
                    "e" => {
                        println! ("{}", f.sqrt());
                        break 'outer;
                    },
                    "f" => {
                        println! ("{}", f * f);
                        break 'outer;
                    },
                    _ => (),
                }
                loop {
                    let mut user_input2 = String::new();
                    println! ("Enter second number:");
                    stdin()
                        .read_line(&mut user_input2)
                        .expect("Program error, crashing");
                    let no_enter_input2: &str = &user_input2.replace("\n", "");
                    match parse_string(&no_enter_input2) {
                        Some(Value::Float(a)) => {
                            match choice.trim_end() {
                                "a" => {
                                    println! ("{}", f + a);
                                    break 'outer;
                                },
                                "b" => {
                                    println! ("{}", f - a);
                                    break 'outer;
                                },
                                "c" => {
                                    println! ("{}", f * a);
                                    break 'outer;
                                },
                                "d" => {
                                    println! ("{}", f / a);
                                    break 'outer;
                                },
                                _ => (),
                            }
                        },
                        None => println! ("Invalid entry! Not a number"),
                    }
                }
            },
            None => println!("Invalid entry! Not a number"),
        }
    }
    return true;
}
