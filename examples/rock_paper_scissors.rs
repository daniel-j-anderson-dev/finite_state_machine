use finite_state_machine::rock_paper_scissors::{
    self, Input as RockPaperScissorsInput, Output as RockPaperScissorsOutput,
};
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), std::io::Error> {
    let mut rock_paper_scissors_finite_state_machine =
        rock_paper_scissors::new_finite_state_machine();
    let mut prompt = "Player one, please input one of the following.\nRock\nPaper\nScissors\n> ";

    loop {
        let input = get_parsed_input::<RockPaperScissorsInput>(prompt)?;
        let output = rock_paper_scissors_finite_state_machine.transition(&input);

        prompt = "Player two, please input one of the following.\nRock\nPaper\nScissors\n> ";

        print("\n")?;

        match output {
            RockPaperScissorsOutput::GameNotOver => continue,
            RockPaperScissorsOutput::Player1Wins => {
                print("Player one wins\n")?;
                break;
            }
            RockPaperScissorsOutput::Player2Wins => {
                print("Player two wins!\n")?;
                break;
            }
            RockPaperScissorsOutput::Tie => {
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

pub fn get_input(prompt: &str) -> Result<String, std::io::Error> {
    print(prompt)?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;
    input.truncate(input.trim_end().len());

    Ok(input)
}

pub fn get_parsed_input<T>(prompt: &str) -> Result<T, std::io::Error>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    loop {
        let input = get_input(prompt)?;
        match input.parse::<T>() {
            Ok(parsed_input) => return Ok(parsed_input),
            Err(parse_error) => writeln!(stdout(), "\nInvalid Input: {}\n", parse_error)?,
        }
    }
}
