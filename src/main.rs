use std::io::{stdin,stdout,Write};
//use std::io::Result;
/*
enum Value {
    Int(isize),
    Float(f64),
}

use Value::*;

fn parse_string(s: &str) -> Option<Value> {
    if let Ok(i) = s.parse() {
        Some(Int(i))
    } else if let Ok(f) = s.parse() {
        Some(Float(f))
    } else {
        None
    }
}
no idea how this code works just referencing for future
*/

fn main() {
    calculator_choice();
    ask_again();
}

fn calculator_choice(){
    let mut cal_choice = String::new();
    let choice_1 = "a";
    let choice_2 = "b";
    let choice_3 = "c";
    let choice_4 = "d";
    let choice_5 = "e";
    let choice_6 = "f";
    loop{
        println!("a) Addition");
        println!("b) Subtraction ");
        println!("c) Multiplication");
        println!("d) Division");
        println!("e) Square Roots");
        println!("f) Squares");
        stdin().read_line(&mut cal_choice);
        if cal_choice == choice_1.parse().unwrap() || cal_choice == choice_2.parse().unwrap() ||
           cal_choice == choice_3.parse().unwrap() || cal_choice == choice_4.parse().unwrap() ||
           cal_choice == choice_5.parse().unwrap() || cal_choice == choice_6.parse().unwrap() {
           if cal_choice == choice_1.parse().unwrap(){
               addition();
               return();
           }
           else if cal_choice == choice_2.parse().unwrap() {
               subtraction();
               return();
           }
           else if cal_choice == choice_3.parse().unwrap() {
               multiplication();
               return();
           }
           else if cal_choice == choice_4.parse().unwrap() {
               division();
               return();
           }
           else if cal_choice == choice_5.parse().unwrap() {
               square();
               return();
           }
           else if cal_choice == choice_6.parse().unwrap() {
               square_root();
               return();
           }
        }
        else {
            println!("Invalid entry! Please choose a, b, c, d, e, or f");
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

}

fn square_root(){

}