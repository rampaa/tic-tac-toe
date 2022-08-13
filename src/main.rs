use std::fmt;
use std::io::stdin;

#[derive(Copy, Clone, PartialEq)]
enum SlotState {
    Empty,
    X,
    O,
}

impl fmt::Display for SlotState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SlotState::Empty => write!(f, " "),
            SlotState::X => write!(f, "X"),
            SlotState::O => write!(f, "O"),
        }
    }
}

struct Board {
    board_state: [SlotState; 9],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "     |     |      \n  {0}  |  {1}  |  {2}\n_____|_____|_____ \n     |     |      \n  {3}  |  {4}  |  {5}\n_____|_____|_____ \n     |     |      \n  {6}  |  {7}  |  {8}\n     |     |      ",
            self.board_state[0],
            self.board_state[1],
            self.board_state[2],
            self.board_state[3],
            self.board_state[4],
            self.board_state[5],
            self.board_state[6],
            self.board_state[7],
            self.board_state[8]
        )
    }
}

fn check_win(board_state: &[SlotState; 9]) -> bool {
    return (board_state[0] != SlotState::Empty
        && ((board_state[0] == board_state[1] && board_state[1] == board_state[2])
            || (board_state[0] == board_state[4] && board_state[4] == board_state[8])
            || (board_state[0] == board_state[3] && (board_state[3] == board_state[6]))))
        || (board_state[6] != SlotState::Empty
            && ((board_state[6] == board_state[7] && board_state[7] == board_state[8])
                || (board_state[6] == board_state[4] && board_state[4] == board_state[2])))
        || (board_state[5] != SlotState::Empty
            && ((board_state[5] == board_state[4] && board_state[4] == board_state[3])
                || (board_state[2] == board_state[5] && board_state[5] == board_state[8])))
        || (board_state[1] != SlotState::Empty
            && (board_state[1] == board_state[4] && board_state[4] == board_state[7]));
}

fn main() {
    let mut board = Board {
        board_state: [SlotState::Empty; 9],
    };

    let mut turn: u8 = 0;
    let mut tie = true;

    while turn < 9 {
        let mut input = String::new();
        println!("{}", board);
        println!("Please enter a number between 1 and 9:");
        stdin().read_line(&mut input).unwrap_or_default();
        let parsed_number = input.trim().parse::<usize>().unwrap_or_default();

        match parsed_number {
            1..=9 => match board.board_state[parsed_number - 1] {
                SlotState::Empty => {
                    let current_mark = if turn % 2 == 0 {
                        SlotState::X
                    } else {
                        SlotState::O
                    };

                    board.board_state[parsed_number - 1] = current_mark;

                    if check_win(&board.board_state) {
                        println!("{}", board);
                        println!("{} won!", current_mark);
                        tie = false;
                        break;
                    }

                    turn += 1;
                }
                _ => {
                    println!("Specified slot is not empty. Please select an empty slot!");
                }
            },
            _ => {
                println!("Invalid input.");
            }
        }
    }

    if tie {
        println!("Game ended in a tie!");
    }
}
