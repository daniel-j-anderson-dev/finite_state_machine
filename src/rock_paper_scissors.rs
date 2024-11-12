use crate::FiniteStateMachine;

pub fn new_finite_state_machine() -> FiniteStateMachine<
    Input,
    Output,
    State,
    fn(&State, &Input) -> State,
    fn(&State, &Input) -> Output,
> {
    FiniteStateMachine::new(next_state, output)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Output {
    GameNotOver,
    Player1Wins,
    Player2Wins,
    Tie,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum State {
    #[default]
    Player1Turn,
    Player2Turn {
        player1_input: Input,
    },
    GameOver {
        player1_input: Input,
        player2_input: Input,
    },
}

#[rustfmt::skip]
pub const fn next_state(state: &State, input: &Input) -> State {
    match (state, input) {
        (State::Player1Turn, Input::Rock)     => State::Player2Turn { player1_input: Input::Rock },
        (State::Player1Turn, Input::Paper)    => State::Player2Turn { player1_input: Input::Paper },
        (State::Player1Turn, Input::Scissors) => State::Player2Turn { player1_input: Input::Scissors },
        (State::Player2Turn { player1_input: Input::Rock },      Input::Rock)     => State::GameOver { player1_input: Input::Rock,     player2_input: Input::Rock },
        (State::Player2Turn { player1_input: Input::Paper },     Input::Paper)    => State::GameOver { player1_input: Input::Paper,    player2_input: Input::Paper },
        (State::Player2Turn { player1_input: Input::Scissors },  Input::Scissors) => State::GameOver { player1_input: Input::Scissors, player2_input: Input::Scissors },
        (State::Player2Turn { player1_input: Input::Paper },     Input::Rock)     => State::GameOver { player1_input: Input::Paper,    player2_input: Input::Rock },
        (State::Player2Turn { player1_input: Input::Scissors },  Input::Rock)     => State::GameOver { player1_input: Input::Scissors, player2_input: Input::Rock },
        (State::Player2Turn { player1_input: Input::Rock },      Input::Paper)    => State::GameOver { player1_input: Input::Rock,     player2_input: Input::Paper },
        (State::Player2Turn { player1_input: Input::Scissors  }, Input::Paper)    => State::GameOver { player1_input: Input::Scissors, player2_input: Input::Paper },
        (State::Player2Turn { player1_input: Input::Rock },      Input::Scissors) => State::GameOver { player1_input: Input::Rock,     player2_input: Input::Scissors },
        (State::Player2Turn { player1_input: Input::Paper },     Input::Scissors) => State::GameOver { player1_input: Input::Paper,    player2_input: Input::Scissors },
        (State::GameOver { .. }, _) => *state,
    }
}

#[rustfmt::skip]
pub const fn output(state: &State, _input: &Input) -> Output {
    match state {
          State::Player1Turn
        | State::Player2Turn { .. }
        => Output::GameNotOver,

          State::GameOver { player1_input: Input::Paper,    player2_input: Input::Rock }
        | State::GameOver { player1_input: Input::Scissors, player2_input: Input::Paper }
        | State::GameOver { player1_input: Input::Rock,     player2_input: Input::Scissors }
        => Output::Player1Wins,

          State::GameOver { player1_input: Input::Rock,     player2_input: Input::Paper }
        | State::GameOver { player1_input: Input::Paper,    player2_input: Input::Scissors }
        | State::GameOver { player1_input: Input::Scissors, player2_input: Input::Rock }
        => Output::Player2Wins,

          State::GameOver { player1_input: Input::Rock,     player2_input: Input::Rock }
        | State::GameOver { player1_input: Input::Paper,    player2_input: Input::Paper }
        | State::GameOver { player1_input: Input::Scissors, player2_input: Input::Scissors }
        => Output::Tie,
    }
}
