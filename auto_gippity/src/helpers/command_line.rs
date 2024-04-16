use crossterm:: {
    style::{ Color, ResetColor, SetForegroundColor },
    ExecutableCommand,
};
use std::io::{ stdin, stdout };

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTesting,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_position: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        // Decide on the print color
        let statement_color = match self {
            PrintCommand::AICall => Color::Cyan,
            PrintCommand::UnitTesting => Color::Magenta,
            PrintCommand::Issue => Color::Red,
        };

        // Print the agent's position in a specific color
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_position);

        // Print the agent's statement in a specific color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        // Reset the color
        stdout.execute(ResetColor).unwrap();
    }
}

pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = std::io::stdout();

    // Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    // Reset the color
    stdout.execute(ResetColor).unwrap();

    // Read user input
    let mut user_response = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read line");

    // Trim whitespace and return
    user_response.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_agent_message() {
        let command = PrintCommand::AICall;
        let agent_position = "Managing Agent";
        let agent_statement = "Test Statement";

        command.print_agent_message(agent_position, agent_statement);
    }
}