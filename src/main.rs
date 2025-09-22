mod todolist;
use std::io;
use std::io::ErrorKind;
use chrono::DateTime;
use chrono::Local;
use rand::prelude::*;
use rand::rngs::OsRng;
use rand::TryRngCore;

// ---Utility Functions---

fn gen_range(from: u32, to: u32) -> Result<u32, io::Error> {
    if from > to {
        return Err(io::Error::new(ErrorKind::InvalidInput, "from > to"));
    }

    let span : u32 = to - from + 1;
    let mut rng = OsRng;

    // Rejection sampling to avoid modulo bias
    let zone : u32 = u32::MAX - (u32::MAX % span);

    loop {
        match rng.try_next_u32() {
            Ok(num) if num < zone => return Ok(from + (num % span)),
            Ok(_) => continue, // retry if num in the "skew zone"
            Err(_) => return Err(io::Error::new(ErrorKind::Other, "RNG failed")),
        }
    }
}

fn main() {
    let mut input : String = String::new();
    // let path = std::path::PathBuf::from("./hi.txt");
    // std::fs::write(path.clone(), String::from("hi!")).unwrap();
    // println!("{}", std::env::current_dir().unwrap().display());
    // // std::fs::File::create(path.clone()).unwrap();
    

    loop {
        clear_screen();
        input.clear();
        println!("The all in one multitool. What shall we use?");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();

        if input == "q" {
            break;
        }

        let input_num: u32 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                invalid_input_msg();
                continue;
            }
        };

        if input_num == 1 {
            calculator();
        }  else if input_num == 2 {
            unit_converter();
        } else if input_num == 3 {
            guessing_game()
        } else if input_num == 4 {
            todolist();
        } else if input_num == 5 {
            password_gen();
        } else if input_num == 6 {
            ropasc();
        } else if input_num == 7 {
            clock();
        }
    }
}


fn clear_screen() {
    print!("\x1B[H\x1B[2J\x1B[3J");
}

fn invalid_input_msg() {
    println!("Invalid input, press enter to continue.");
    let mut a: String = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Why ;c");
}

fn calculator() {
    let mut input : String = String::new();
    loop {
        clear_screen();
        input.clear();

        println!("Calculator, enter a number, enter q to exit.");
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
        input = input.trim().to_string();

        if input == "q" {
            break;
        }

        let input_a: f64 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                invalid_input_msg();
                continue;
            }
        };

        println!("Enter the second number");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
        input = input.trim().to_string();

        let input_b: f64 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                invalid_input_msg();
                continue;
            }
        };

        println!("Enter operation: % / * + -");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
        input = input.trim().to_string();

        if input == "+" {
            println!("{}", input_a + input_b);
        } else if input == "-" {
            println!("{}", input_a - input_b);
        } else if input == "*" {
            println!("{}", input_a * input_b);
        } else if input == "/" {
            println!("{}", input_a / input_b);
        } else if input == "%" {
            println!("{}", input_a % input_b);
        }
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
    }
}

fn unit_converter() {
    let mut input : String = String::new();
    loop {
        clear_screen();
        input.clear();

        println!("Unit converter, enter a number, enter q to exit.");
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
        input = input.trim().to_string();

        if input == "q" {
            break;
        }

        let input_a: f64 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                invalid_input_msg();
                continue;
            }
        };

        println!("Enter conversion type:\n1.inch->cm\n2.cm->inch\n3.mile->km\n4.km->mile\n5.m->ft\n6.ft->m");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
        input = input.trim().to_string();

        let input_b: u8 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                invalid_input_msg();
                continue;
            }
        };

        if input_b == 1 {
            println!("{}", input_a * 2.54)
        } else if input_b == 2 {
            println!("{}", input_a / 2.54)
        } else if input_b == 3 {
            println!("{}", input_a * 1.609)
        } else if input_b == 4 {
            println!("{}", input_a / 1.609)
        } else if input_b == 5 {
            println!("{}", input_a * 3.28)
        } else if input_b == 6 {
            println!("{}", input_a / 3.28)
        }
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
    }
}

fn guessing_game() {
    let mut rng = rand::rng();
    let mut input = String::new();
    loop {
        clear_screen();
        input.clear();

        println!("Guessing game, type q to exit, or something else to continue. ");
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
        input = input.trim().to_string();

        if input == "q" {
            break;
        }

        let answer: u32 = rng.random_range(0..=100);
        let mut guess : u32;

        loop {
            println!("Enter your guess: ");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Why ;c");
            input = input.trim().to_string();

            guess = match input.parse() {
                Ok(num) => num,
                Err(_) => {
                    invalid_input_msg();
                    continue;
                }
            };
            
            if guess == answer {
                println!("You guessed {}! That's correct!", guess);
                io::stdin()
                    .read_line(&mut input)
                    .expect("Why ;c");
                break;
            } else if guess < answer {
                println!("Bigger.");
            } else if guess > answer {
                println!("Smaller.");
            }
        }
    }
}

fn todolist() {
    let mut list : todolist::List = todolist::List::new();
    let mut input : String = String::new();
    loop {
        clear_screen();
        input.clear();
        
        println!("Todolist, choose an option:");
        println!("0. List tasks");
        println!("1. Add task");
        println!("2. Remove task");
        println!("3. Mark task");
        println!("Type q to exit");
        
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
        input = input.trim().to_string();
        
        if input == "q" {
            break;
        }

        let input_a: u8 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                invalid_input_msg();
                continue;
            }
        };
        
        if input_a == 0 {
            for item in &list.map {
                println!("Task: {}  Completed: {}", item.1.title, item.1.completed)
            }
            io::stdin()
                .read_line(&mut input)
                .expect("Why ;c");
        }
        
        if input_a == 1 {
            println!("Name the task:");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Why ;c");
            input = input.trim().to_string();
            
            let item : todolist::ListItem = todolist::ListItem {
                title : input.clone(),
                completed : false
            };
            list.add(item);
        }
        
        if input_a == 2 {
            println!("What task to remove?");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Why ;c");
            input = input.trim().to_string();
            
            list.remove(input.clone());
        }
        
        if input_a == 3 {
            println!("What to mark?");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Why ;c");
            input = input.trim().to_string();
            
            list.mark(&input.clone());
        }
    }
}

fn password_gen() {
    let mut input : String = String::new();
    let mut char_string : String = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=_+,./<>?'\";:[]{}\\`~!@#$%^&*()");
    let mut length : u32 = 0;
    let mut quantity : u32 = 0;
    loop {
        clear_screen();
        input.clear();
        println!("Password generator, enter q to exit. Type characters string or type nothing to use the default one.");
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
        input = input.trim().to_string();

        if input == "q" {
            break;
        }
        else if input != "" {
            char_string = String::from(input.trim());
        }

        println!("Now enter the length of the passwords.");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
        input = input.trim().to_string();

        length = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                invalid_input_msg();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Why ;c");
                continue;
            }
        };

        println!("Now enter the quantity of the passwords.");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
        input = input.trim().to_string();

        quantity = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                invalid_input_msg();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Why ;c");
                continue;
            }
        };

        for i in 0..quantity {
            print!("{}: ", i + 1);
            let mut temp : String = String::new();
            for _ in 0..length {
                let index : usize = match gen_range(0, (char_string.len() - 1) as u32) {
                    Ok(num) => num as usize,
                    Err(err) => {
                        println!("Error: {err}");
                        continue;
                    }
                };
                temp.push(char_string.chars().nth(index).unwrap());
            }
            println!("{}", temp);
        }
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
    }
}

fn ropasc() {
    let mut input : String = String::new();
    loop {
        clear_screen();
        input.clear();

        println!("Rock Paper Scissors, enter q to exit, select:\nr - rock\np - paper\ns - scissors");
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
        input = input.trim().to_string();

        if input == "q" {
            break;
        }

        let index : usize = match gen_range(0, (3 - 1) as u32) {
            Ok(num) => num as usize,
            Err(err) => {
                println!("Error: {err}");
                continue;
            }
        };

        let selection : String = String::from("spr").chars().nth(index).unwrap().to_string();

        if selection == input {
            println!("Tie!");
        } else if  selection == "r" && input == "p"
                   || selection == "s" && input == "r"
                   || selection == "p" && input == "s" {
            println!("You win!");
        } else {
            println!("You lose!");
        }
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
    }
}

fn clock() {
    let mut input : String = String::new();
    loop {
        clear_screen();
        input.clear();

        println!("Clock utility. To get help type h, to exit type q.");
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
        input = input.trim().to_string();

        let local_datetime : DateTime<Local> = Local::now();

        if input == "q" {
            break;
        } else if input == "h" {
            println!("Help:");
            println!("h - help");
            println!("q - quit");
            println!("t - time");
            println!("d - date");
            println!("s - timestamp");
        } else if input == "t" {
            println!("{}", local_datetime.format("%H:%M:%S"));
        } else if input == "d" {
            println!("{}", local_datetime.format("%Y-%m-%d"));
        } else if input == "s" {
            println!("{}", local_datetime.timestamp());
        }
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
    }
}