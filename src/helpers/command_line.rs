use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

// Get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // Print question in an espedific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    // reset color
    stdout.execute(ResetColor).unwrap();

    // read user input
    let mut user_response = String::new();
    stdin()
    .read_line(&mut user_response)
    .expect("Failed to read line");

    // trim whitespaces and return
    return user_response.trim().to_string();
}