use chrono::prelude::{Local};
use colored::*;

const DIGITS : [[&str; 11]; 7] = [
    ["┏━┓ ","  ╻  "," ┏━┓ ", " ┏━┓ "," ╻ ╻ "," ┏━┓ "," ┏   "," ┏━┓ "," ┏━┓ "," ┏━┓ ","   "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ "," ╻ "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ ","   "],
    ["┃ ┃ ","  ┃  "," ┏━┛ ", " ┣━┫ "," ┗━┫ "," ┗━┓ "," ┣━┓ ","   ┃ "," ┣━┫ "," ┗━┫ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ╹ "],
    ["┗━┛ ","  ╹  "," ┗━━ ", " ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   "],
];


fn print_time() {
    let local_time: String = Local::now().format("%H:%M:%S").to_string();
    let time_chars: Vec<char> = local_time.chars().collect();
    
    for i in 0..DIGITS.len() {
        let mut line = String::new();

        // For each character in the time string...
        for &c in &time_chars {
            // If the character is a digit, add the digit to the line, otherwise, if it is a colon, add the colon to the line.
            if let Some(digit) = c.to_digit(10) {
                line += DIGITS[i][digit as usize];
            } else if c == ':' {
                line += DIGITS[i][10];
            }
        }
        println!("{}", line.red());
    }
}


/*
    system functionality & explanation
    - get current time
    - get current epoch

    - if the current epoch time is different from the previous epoch time, around a second
        - update the current time
        - update the current epoch time
        - print the current time

    - repeat
*/
fn main() {
    let mut now = Local::now();
    let mut now_epoch = now.timestamp();

    loop {
        if Local::now().timestamp() != now_epoch {
            now = Local::now();
            now_epoch = now.timestamp();
            print_time();
        }
    }
}
