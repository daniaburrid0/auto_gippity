use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum PrintCommand {
    AiCall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_position: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        // Deside on the print color 
        let statement_color = match self {
            PrintCommand::AiCall => Color::Cyan,
            PrintCommand::UnitTest => Color::Magenta,
            PrintCommand::Issue => Color::Red,
        };

        // Print agent statement in an espedific color
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_position);

        // Change color to print the statement
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        // reset color
        stdout.execute(ResetColor).unwrap();
    }
    
}

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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_print_agent_message() {
        PrintCommand::AiCall.print_agent_message("Managin Agent", "test test test test");
    }
}