mod todolist;
use std::io;
use rand::prelude::*;

fn main() {
    let mut input : String;

    loop {
        clear_screen();
        input = String::new();
        println!("The all in one multitool. What shall we use?");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "q" {
            break;
        }

        let input_num: u32 = match input.trim().parse() {
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
        }
    }
}


fn clear_screen() {
    print!("\x1B[H\x1B[2J\x1B[3J");
}

fn invalid_input_msg() {
    println!("Invalid input, press enter to continue.");
    let mut a = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Why ;c");
}

fn calculator() {
    loop {
        clear_screen();
        println!("Calculator, enter a number, enter q to exit.");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
        if input.trim() == "q" {
            break;
        }

        let input_a: f64 = match input.trim().parse() {
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

        let input_b: f64 = match input.trim().parse() {
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

        if input.trim() == "+" {
            println!("{}", input_a + input_b);
        } else if input.trim() == "-" {
            println!("{}", input_a - input_b);
        } else if input.trim() == "*" {
            println!("{}", input_a * input_b);
        } else if input.trim() == "/" {
            println!("{}", input_a / input_b);
        } else if input.trim() == "%" {
            println!("{}", input_a % input_b);
        }
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
    }
}

fn unit_converter() {
    loop {
        clear_screen();
        println!("Unit converter, enter a number, enter q to exit.");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
        if input.trim() == "q" {
            break;
        }

        let input_a: f64 = match input.trim().parse() {
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

        let input_b: u8 = match input.trim().parse() {
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
        if input.trim() == "q" {
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
            guess = match input.trim().parse() {
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
    let mut input = String::new();
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
        if input.trim() == "q" {
            break;
        }

        let input_a: u8 = match input.trim().parse() {
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
            input.clear();
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
            let item : todolist::ListItem = todolist::ListItem {
                title : String::from(input.trim()),
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
            list.remove(String::from(input.trim()));
        }
        
        if input_a == 3 {
            println!("What to mark?");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Why ;c");
            list.mark(&String::from(input.trim()));
        }
    }
}