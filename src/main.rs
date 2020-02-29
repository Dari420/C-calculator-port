use std::io::{stdin, Read};
use std::time::Duration;
use std::thread::{sleep};
use std::process::exit;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::convert::TryFrom;

enum Value { //enums to match on
Float(f64),
}

enum Value2 {
    Uns(usize),
}

fn parse_string2(s: &str) -> Option<Value2> { //parses for an int
    if let Ok(u) = s.parse() {
        Some(Value2::Uns(u))
    } else {
        None
    }
}

fn parse_string(s: &str) -> Option<Value> { //parses for a float
    if let Ok(f) = s.parse() {
        Some(Value::Float(f))
    } else {
        None
    }
}

fn main() {
    println! ("Welcome to Dari's rust calculator");
    loop {
        calculator_choice();
        ask_again();
    }
}

fn calculator_choice() { //ask for choice and pass it down to calculation function
    loop {
        let mut cal_choice = String::new();
        println!("Choose operation:");
        println!("a) Prime Numbers");
        println!("b) Square Roots");
        println!("c) Squares");
        println!("d) Multiplication Table");
        println!("e) Addition");
        println!("f) Subtraction");
        println!("g) Multiplication");
        println!("h) Division");
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

fn ask_again(){ //ask again and either loop to asking for a choice or kill program
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
                sleep(Duration::new(0, 900000000));
                exit(0);
            },
            _ => println! ("Invalid entry! Please choose y or n")
        }
    }
}

fn ask_write_file() -> bool { //ask if user wants to write output to file
    loop {
        let mut write_or_not = String::new();
        println! ("Write to file? y/n (warning: this overrides the current output)");
        stdin()
            .read_line(&mut write_or_not)
            .expect("Invalid entry! crashing");
        match write_or_not.trim_end() {
            "y" => {
                break true;
            },
            "n" => {
                println! ("Not writing to file");
                break false;
            },
            _ => println! ("Invalid entry! Please choose y or n")
        }
    }
}

fn calculations(choice: String) -> bool{ //checks for choice and outputs based on choice
    if choice.trim_end() == "b" || choice.trim_end() == "c" || //if the choice isn't in listed then return false and tell the choice function to ask again
        choice.trim_end() == "e" || choice.trim_end() == "f" ||
        choice.trim_end() == "g" || choice.trim_end() == "h" { //if it is, go in a loop until the output has been given
        'outer: loop {
            //let mut contents = fs::read_to_string(path)
            //.expect("Error: file doesnt exist");
            let mut user_input1 = String::new(); //store number 1
            println! ("Enter first number:");
            stdin()
                .read_line(&mut user_input1)
                .expect("Program error, crashing");
            let no_enter_input1: &str = &user_input1.replace("\r\n", "").replace("\n", ""); //remove newline chars
            match parse_string(&no_enter_input1) { //parse for float
                Some(Value::Float(f)) => {
                    match choice.trim_end() { //check choice
                        "b" => {
                            println! ("{}", f.sqrt());
                            if ask_write_file() {
                                write_file_primitive(f.sqrt());
                            }
                            break 'outer;
                        },
                        "c" => {
                            println! ("{}", f * f);
                            if ask_write_file() {
                                write_file_primitive(f * f);
                            }
                            break 'outer;
                        },
                        _ => (),
                    }
                    loop {
                        let mut user_input2 = String::new(); //if it's a two number option, ask for number 2
                        println! ("Enter second number:");
                        stdin()
                            .read_line(&mut user_input2)
                            .expect("Program error, crashing");
                        let no_enter_input2: &str = &user_input2.replace("\r\n", "").replace("\n", "");
                        match parse_string(&no_enter_input2) {
                            Some(Value::Float(a)) => {
                                match choice.trim_end() { //check which specific option was chosen
                                    "e" => {
                                        println! ("{}", f + a);
                                        if ask_write_file() {
                                            write_file_primitive(f + a);
                                        }
                                        break 'outer;
                                    },
                                    "f" => {
                                        println! ("{}", f - a);
                                        if ask_write_file() {
                                            write_file_primitive(f - a);
                                        }
                                        break 'outer;
                                    },
                                    "g" => {
                                        println! ("{}", f * a);
                                        if ask_write_file() {
                                            write_file_primitive(f * a);
                                        }
                                        break 'outer;
                                    },
                                    "h" => {
                                        println! ("{}", f / a);
                                        if ask_write_file() {
                                            write_file_primitive(f / a);
                                        }
                                        break 'outer;
                                    },
                                    _ => (),
                                }
                            },
                            None => println! ("Invalid entry! Not a number"),  //tell user to enter a number
                        }
                    }
                },
                None => println!("Invalid entry! Not a number"),
            }
        }
        return true;
    }
    else if choice.trim_end() == "d" ||  choice.trim_end() == "a" { //if choice is int-specific then go here
        'outer_2: loop {
            let mut user_input1b = String::new();
            println!("Enter first number:");
            stdin()
                .read_line(&mut user_input1b)
                .expect("Program error, crashing");
            let no_enter_input1b: &str = &user_input1b.replace("\r\n", "").replace("\n", "");
            match parse_string2(&no_enter_input1b) { //parses for an int
                Some(Value2::Uns(u)) => {
                    let path = Path::new("output.txt");
                    let mut data_to_write = String::new();
                    let mut multiplier_rows: isize = 0;
                    let mut multiplier_cols: isize = 0;
                    let mut product: isize;
                    match choice.trim_end() {
                        "d" => {
                            for _rows in 0usize..u {
                                multiplier_rows += 1;
                                multiplier_cols = 0;
                                for _columns in 0usize..u {
                                    multiplier_cols += 1;
                                    product = &multiplier_rows * multiplier_cols;
                                    print!("{}, ", product);
                                    &data_to_write.push_str(&product.to_string());
                                    &data_to_write.push_str(", ");
                                }
                                &data_to_write.push_str("\n");
                                print!("\n")
                            }
                            if ask_write_file() { //I am not using the function here because this is a string so i cannot apply the typical function here.
                                let mut path_input= String::new(); //create empty path
                                println!("Enter filename:");
                                stdin().read_line(&mut path_input)
                                    .expect("error reading filename line");
                                path_input.truncate(path_input.len() - 1);
                                let path_input_final = format! ("./outputs/{}.txt", path_input);
                                println! ("Name: {}", path_input_final);
                                fs::create_dir_all("outputs")
                                    .expect("error making output directory: directory either exists or permissions are not granted");
                                let path = Path::new(&path_input_final);
                                println! ("Writing file..");
                                sleep(Duration::new(0, 600000000));
                                File::create(&path)
                                    .expect("Error writing file (hint: maybe denied permissions?");
                                fs::write(&path, &data_to_write)
                                    .expect("Error writing file (hint: maybe denied permissions?"); //write val
                            }
                            println!("Done");
                            break 'outer_2;
                        },
                        "a" => {
                            println! ("Calculating Primes...");
                            sleep(Duration::new(0, 300000000));
                            prime_sieve(u, false);
                            if ask_write_file() {
                                let mut path_input= String::new(); //create empty path
                                println!("Enter filename:");
                                stdin().read_line(&mut path_input)
                                    .expect("error reading filename line");
                                path_input.truncate(path_input.len() - 1);
                                let path_input_final = format! ("./outputs/{}.txt", path_input);
                                println! ("Name: {}", path_input_final);
                                fs::create_dir_all("outputs")
                                    .expect("error making output directory: directory either exists or permissions are not granted");
                                let path = Path::new(&path_input_final);
                                println! ("Writing file..");
                                sleep(Duration::new(0, 600000000));
                                File::create(&path)
                                    .expect("Error writing file (hint: maybe denied permissions?");
                                fs::write(&path, (format! ("{:?}", prime_sieve(u, true))).replace("[", "").replace("]", ""))
                                    .expect("Error writing file (hint: maybe denied permissions?");
                            }
                            break 'outer_2;
                        },
                        _ => println! ("fatal error")
                    }
                },
                None => println!("Invalid entry! Not a number"),
            }
        }
        return true
    }
    else {
        println! ("Invalid input! Please choose a, b, c, d, e, f, or g");
        return false;
    }
}
/*
fn write_file(f: f64) {
    let path = Path::new("output.txt");
    if ask_write_file() {
        File::create(path)
            .expect("Error writing file (hint: maybe denied permissions?");
        let mut contents_a = fs::read_to_string(path)
            .expect("Error: file doesnt exist"); //store current file data
        fs::write(path, (f).to_string())
            .expect("Error writing file (hint: maybe denied permissions?"); //write val
        let mut contents_b = fs::read_to_string(path)
            .expect("Error: file doesnt exist"); //store val
        fs::write(path, contents_a.push_str(contents_b.as_str()).push_str("\n"))
            .expect("Error writing file (hint: maybe denied permissions?"); //write previous file data, val, and a newline
        sleep(Duration::new(0, 800000000));
    }
}
to troubleshoot later
*/
fn prime_sieve(limit: usize, print: bool) -> Vec<usize> {
    let mut is_prime = vec![true; limit+1];
    is_prime[0] = false;
    if limit >= 1 { is_prime[1] = false }
    for num in 2..limit+1 {
        if is_prime[num] {
            let mut multiple = num*num;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += num;
            }
        }
    }
    if print == false {
        let value: Vec<usize> = is_prime.iter().enumerate()
            .filter_map(|(pr, &is_pr)| if is_pr { Some(pr) } else { None })
            .collect();
        let bracket = format!("{:?}", value);
        let no_bracket = bracket.replace("[", "").replace("]", "");
        println!("{}", no_bracket);
        println!("Found {} primes", &is_prime.iter().filter(|&n| *n == true).count());
        sleep(Duration::new(0, 300000000));
    }
    else {
    }
    is_prime.iter().enumerate()
        .filter_map(|(pr, &is_pr)| if is_pr {Some(pr)} else {None} )
        .collect()
}

fn write_file_primitive (result: f64) { //write result to file
    let mut path_input= String::new(); //create empty path
    println!("Enter filename:");
    stdin().read_line(&mut path_input)
        .expect("error reading filename line");
    path_input.truncate(path_input.len() - 1); //remove newline
    let path_input_final = format! ("./outputs/{}.txt", path_input); //add extension
    println! ("Name: {}", path_input_final);
    fs::create_dir_all("outputs")
        .expect("error making output directory: directory either exists or permissions are not granted");
    let path = Path::new(&path_input_final);
    println! ("Writing file..");
    sleep(Duration::new(0, 600000000));
    File::create(&path)
        .expect("Error writing file (hint: maybe denied permissions?");
    fs::write(&path, result.to_string())
        .expect("Error writing file (hint: maybe denied permissions?"); //write val
}
