#![allow(unused)]
#![feature(core_intrinsics)]
#![feature(repr128)]
use std::io::{stdin,stdout,Write};
use std::io::Result;
use std::str::FromStr;

enum Value {
    Int(isize),
    Float(f64),
}

use std::intrinsics::sqrtf64;


// This function is not limited to just numeric types but rather anything that implements the FromStr trait.
fn parsable<T: FromStr>(s: &str) -> bool {
    s.parse::<T>().is_ok()
}

fn parse_string(s: &str) -> Option<Value> {
    if let Ok(i) = s.parse() {  // inferred as isize from next line
        Some(Value::Int(i))
    } else if let Ok(f) = s.parse() {
        Some(Value::Float(f))
    } else {
        None
    }
}

fn main() {
    loop {
        calculator_choice();
        ask_again();
    }
}

fn calculator_choice() {
    let mut cal_choice = String::new();
    loop {
        println!("a) Addition");
        println!("b) Subtraction ");
        println!("c) Multiplication");
        println!("d) Division");
        println!("e) Square Roots");
        println!("f) Squares");
        stdin()
            .read_line(&mut cal_choice)
            .expect("invalid input! crashing"); {
            match cal_choice.trim_end() {
                "a" => {
                    addition();
                    break;
                },
                "b" => {
                    subtraction();
                    break;
                },
                "c" => {
                    multiplication();
                    break;
                },
                "d" => {
                    division();
                    break;
                },
                "e" => {
                    square_root();
                    break;
                },
                "f" => {
                    square();
                    break;
                },
                _ => println!("Invalid entry! Please choose a, b, c, ,d, e, or f"),
            }
        }
    }
}

fn ask_again(){

}

fn addition(){

}

fn subtraction(){

}

fn multiplication(){

}
fn division(){

}

fn square(){
    let mut user_input = String::new();
    let mut square: f64;
    loop {
        println! ("enter number:");
        stdin()
            .read_line(&mut user_input)
            .expect("Program error, crashing");
        /*
        let mut parsed_user_input = &user_input.to_owned().parse::<f32>();
        if (parsed_user_input.is_err()) {
            println!("Invalid entry! not a number");
        }
        else {
            square = parsed_user_input.parse().unwrap() * parsed_user_input.parse().unwrap();
            println! ("{}", square);
            break;
        }*/
        match parse_string(&user_input.to_string()) {
            Some(Value::Int(i)) => {
                i * i;
            },
            Some(Value::Float(f)) => {
                f * f;
            },
            None => println!("Invalid entry! Not a number"),
        }
    }
}

fn square_root(){
    let mut user_input = String::new();
    loop {
        println! ("enter number:");
        stdin()
            .read_line(&mut user_input)
            .expect("Program error, crashing");
        match parse_string(&user_input) {
            Some(Value::Float(f)) => {
                let mut square_root: f64;
                square_root = f.sqrt();
                println! ("{}", square_root);
                break;
            },
            Some(Value::Int(i)) => {
                let mut square_root2: f64;
                square_root2 = (i as f64).sqrt();
                println! ("{}", square_root2);
                break;
            }
            None => println!("Invalid entry! not a number"),
            _ => {}
        }
    }
}