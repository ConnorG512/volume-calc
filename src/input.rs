use std::io;

pub fn take_user_input() -> f64 {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    
    user_input.trim().parse::<f64>().expect("Invalid Input")
}
