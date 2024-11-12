use finite_state_machine::rock_paper_scissors;
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), std::io::Error> {
    let mut rock_paper_scissors_finite_state_machine =
        rock_paper_scissors::new_finite_state_machine();
    let mut prompt = "Player one, please input one of the following.\nRock\nPaper\nScissors\n> ";

    loop {
        let input = get_input(prompt)?;
        let output = rock_paper_scissors_finite_state_machine.transition(&input);

        print("\n")?;

        match output {
            rock_paper_scissors::Output::GameNotOver => {
                prompt = "Player two, please input one of the following.\nRock\nPaper\nScissors\n> "
            }
            rock_paper_scissors::Output::Player1Wins => {
                print("Player one wins\n")?;
                break;
            }
            rock_paper_scissors::Output::Player2Wins => {
                print("Player two wins!\n")?;
                break;
            }
            rock_paper_scissors::Output::Tie => {
                print("It's a tie!\n")?;
                break;
            }
        }
    }

    Ok(())
}

pub fn print(message: &str) -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    stdout.write_all(message.as_bytes())?;
    stdout.flush()?;
    Ok(())
}

pub fn get_input(prompt: &str) -> Result<rock_paper_scissors::Input, std::io::Error> {
    loop {
        print(prompt)?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;
        input.truncate(input.trim_end().len());

        match input.to_lowercase().as_str() {
            "rock" => return Ok(rock_paper_scissors::Input::Rock),
            "paper" => return Ok(rock_paper_scissors::Input::Paper),
            "scissors" => return Ok(rock_paper_scissors::Input::Scissors),
            _ => print("\nInvalid Input: valid inputs are \"Rock\", \"Paper\", or \"Scissors\"\n")?,
        }
    }
}
