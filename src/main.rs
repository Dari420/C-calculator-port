use std::io::stdin;
use std::time::Duration;
use std::thread::{sleep};
use std::process::exit;
use std::fs;
use std::fs::File;
use std::path::Path;
use pbr::ProgressBar;

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

fn calculations(choice: String) -> bool{ //checks for choice and outputs based on choice
    if choice.trim_end() == "b" || choice.trim_end() == "c" || //if the choice isn't in listed then return false and tell the choice function to ask again
        choice.trim_end() == "e" || choice.trim_end() == "f" ||
        choice.trim_end() == "g" || choice.trim_end() == "h" { //if it is, go in a loop until the output has been given
        'outer: loop {
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
                                write_file((f.sqrt()).to_string());
                            }
                            break 'outer;
                        },
                        "c" => {
                            println! ("{}", f * f);
                            if ask_write_file() {
                                write_file((f * f).to_string());
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
                                            write_file((f + a).to_string());
                                        }
                                        break 'outer;
                                    },
                                    "f" => {
                                        println! ("{}", f - a);
                                        if ask_write_file() {
                                            write_file((f - a).to_string());
                                        }
                                        break 'outer;
                                    },
                                    "g" => {
                                        println! ("{}", f * a);
                                        if ask_write_file() {
                                            write_file((f * a).to_string());
                                        }
                                        break 'outer;
                                    },
                                    "h" => {
                                        println! ("{}", f / a);
                                        if ask_write_file() {
                                            write_file((f / a).to_string());
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
            println!("{}", if choice.trim_end() == "a" {"Enter prime limit:"} else {"Enter table limit:"});
            stdin()
                .read_line(&mut user_input1b)
                .expect("Program error, crashing");
            let no_enter_input1b: &str = &user_input1b.replace("\r\n", "").replace("\n", "");
            match parse_string2(&no_enter_input1b) { //parses for an int
                Some(Value2::Uns(u)) => {
                    let mut data_to_write_table = String::new();
                    let mut multiplier_rows: usize;
                    let mut multiplier_cols: usize;
                    let mut product: usize;
                    let mut result;
                    match choice.trim_end() {
                        "d" => {
                            for rows in 0usize..(u + 1) {
                                for columns in 0usize..(u + 1) {
                                    /*
                                        essentially to get the outer multipliers, force the multiplier for columns to be 1 at the start,
                                        and then multiply with the rows to get the extra number. Setting it to 1 instead of 0 allows it
                                        to do the outer multipliers at the very start, and then subsequently matching the
                                        iteration number to create the normal multiplication table. Works both ways.
                                        Also checks if it's the very first iteration for both loops so it can set the X.
                                    */
                                    multiplier_rows = rows; //set the multiplier to the current incrementation in the range
                                    if rows == 0 { //if it's the first incrementation, set the multiplier to 1 for the outside
                                        multiplier_rows = 1
                                    }

                                    multiplier_cols = columns;
                                    if columns == 0 { //same story
                                        multiplier_cols = 1
                                    }
                                    product = &multiplier_rows * multiplier_cols;
                                    if rows == 0 && columns == 0 { //if it's the first iteration on both for loops, start with X. Otherwise, go with number
                                        result = String::from("X");
                                    }
                                    else {
                                        result = product.to_string();
                                    }
                                    print!("{}\t", result);
                                    &data_to_write_table.push_str(&result);
                                    &data_to_write_table.push_str("\t");
                                }
                                &data_to_write_table.push_str("\n");
                                print!("\n")
                            }
                            if ask_write_file() {
                                write_file(data_to_write_table);
                            }
                            break 'outer_2;
                        },
                        "a" => {
                            println! ("Calculating Primes...");
                            sleep(Duration::new(0, 300000000));
                            prime_slow_process(u); //uncomment if you are running out of memory
                            //prime_fast_process(u); //comment out if you use slow version
                            break 'outer_2;
                        },
                        _ => println!("fatal error")
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

fn prime_fast(limit: usize, print: bool) -> Vec<usize> {
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

fn prime_fast_process(val: usize) {
    prime_fast(val, false);
    if ask_write_file() {
        write_file(format! ("{:?}", prime_fast(val, true)).replace("[", "").replace("]", ""));
    }
}

fn prime_slow(number: usize) -> bool{
    for i in 2..(number/2) {
        if number % i == 0 {
            return false
        }
    }
    return true
}

fn prime_slow_process(val: usize) {
    let mut data_to_write_prime = String::new();
    let mut bar = ProgressBar::new(val as u64);
    let mut j = 0;
    let mut k = 1;
    while j < val + 1 {
        k += 1;
        if prime_slow(k) {
            let prime= k.to_string();
            data_to_write_prime.push_str(&prime);
            data_to_write_prime.push_str(", ");
            j += 1;
        }
        bar.inc();
    }
    bar.finish();
    println!();
    data_to_write_prime.truncate(data_to_write_prime.len() - 2);
    println!("{}", data_to_write_prime.replace("4, ", ""));
    if ask_write_file() {
        write_file(data_to_write_prime);
    }
}

fn write_file (result: String) { //write result to file
    let mut path_input= String::new(); //create empty path
    println!("Enter filename:");
    stdin()
        .read_line(&mut path_input) //read input
        .expect("error reading filename line");
    path_input.truncate(path_input.len() - 2); //remove newline; for windows its -2, for linux/mac its -1
    println! ("Name: {}", format! ("{}.txt", path_input)); //tell user name
    let path_input_final = format! ("./outputs/{}.txt", path_input); //add extension and folder
    fs::create_dir_all("outputs") //create folder
        .expect("error making output directory: directory either exists or permissions are not granted");
    let path = Path::new(&path_input_final); //create path from path
    println! ("Writing file..");
    sleep(Duration::new(0, 300000000)); //Slight delay to not make program look choppy
    File::create(&path) //create file from path
        .expect("Error writing file (hint: maybe denied permissions?");
    fs::write(&path, result) //write contents
        .expect("Error writing file (hint: maybe denied permissions?"); //write val
    println!("Done :D");
}

fn ask_write_file() -> bool { //ask if user wants to write output to file
    loop {
        let mut write_or_not = String::new(); //new string
        println! ("Write to file? y/n (warning: this will override a file with the same name in the outputs folder)");
        stdin() //read choice
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

//deprecated code
/* Old multiplication table
for _rows in 0usize..u {
    multiplier_rows += 1;
    multiplier_cols = 0;
    for _columns in 0usize..u {
        multiplier_cols += 1;
        product = &multiplier_rows * multiplier_cols;
        print!("{}, ", product);
        &data_to_write_table.push_str(&product.to_string());
        &data_to_write_table.push_str(", ");
    }
    &data_to_write_table.push_str("\n");
    print!("\n")
}
*/