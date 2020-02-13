#![allow(unused)]
#![feature(core_intrinsics)]
#![feature(repr128)]
use std::io::{stdin,stdout,Write};
use std::io::Result;
use std::str::FromStr;

enum Value {
    Float(f64),
}

use Value::*;
use std::intrinsics::sqrtf64;



fn parse_string(s: &str) -> Option<Value> {
     if let Ok(f) = s.parse() {
        Some(Float(f))
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
    let choice_1 = "a";
    let choice_2 = "b";
    let choice_3 = "c";
    let choice_4 = "d";
    let choice_5 = "e";
    let choice_6 = "f";
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
                choice_1 if true => {
                    addition();
                    break;
                }
                choice_2 if true => {
                    subtraction();
                    break;
                },
                choice_3 if true => {
                    multiplication();
                    break;
                },
                choice_4 if true => {
                    division();
                    break;
                },
                choice_5 if true => {
                    square_root();
                    break;
                },
                choice_6 if true => {
                    square();
                    break;
                },
                _ => println!("Invalid entry! Please choose a, b, c, ,d, e, or f"),
            }
        }
    }
}
/*
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
            match cal_choice.trim_right() {
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
*/
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
        match parse_string(&user_input) {
            Some(Float(f)) => {
                let corrected_user_input = f64::from_str(&user_input).unwrap();
                square = corrected_user_input * corrected_user_input;
                println! ("{}", square);
                break;
            },
            None => println!("Invalid entry! not a number"),
        }
    }
}

fn square_root(){
    let mut user_input = String::new();
    let mut square_root: f64;
    loop {
        println! ("enter number:");
        stdin()
            .read_line(&mut user_input)
            .expect("Program error, crashing");
        match parse_string(&user_input) {
            Some(Float(f)) => {
                let corrected_user_input = f64::from_str(&user_input).unwrap();
                square_root = corrected_user_input.sqrt();
                println! ("{}", square_root);
                break;
            },
            None => println!("Invalid entry! not a number"),
        }
    }
}