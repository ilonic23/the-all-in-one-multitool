use std::io;

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

        println!("Enter conversion type:\n1.inch->cm\n2.cm->inch\n3.mile->km\n4.km->mile");
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
        }
        io::stdin()
            .read_line(&mut input)
            .expect("Why ;c");
    }
}