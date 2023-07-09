mod game_error;
mod game_move;
mod game_result;
mod game_type;
mod main_action;
mod app_step;

use std::io::{self};

use crate::game_move::GameMove;
use crate::game_result::GameResult;
use crate::game_type::GameType;
use crate::AppStep::MainMenu;
use console::{style, Term};
use rand::rngs::ThreadRng;
use rand::Rng;
use crate::app_step::AppStep;

fn write_overview(term: &Term) {
    let overview = "Simple CLI rock-paper-scissors game with few variations: Spock-lizard and fire-water.";
    term.write_line(&format!("{}", style(overview).yellow())).unwrap();
    write_empty_line(term);
}

fn write_empty_line(term: &Term) {
    term.write_line("").unwrap();
}

fn get_main_menu(term: &Term) -> main_action::MainAction {
    term.write_line(&format!("{}", style(main_action::get_main_actions_menu()).cyan())).unwrap();
    loop {
        match main_action::choose_action(term.read_char().unwrap()) {
            Ok(x) => {
                return x;
            }
            Err(err) => {
                term.write_line(&format!("{}", style(err.0).red())).unwrap();
            }
        };
    }
}

fn parse_player_move(term: &Term, game_type: &GameType) -> GameMove {
    term.write_line(&format!("{}", style(game_type.get_move_menu()).cyan())).unwrap();
    loop {
        match GameMove::parse_game_move(game_type, term.read_char().unwrap()) {
            Ok(x) => {
                return x;
            }
            Err(err) => {
                term.write_line(&format!("{}", style(err.0).red())).unwrap();
            }
        }
    }
}

fn get_computer_move(game_type: &GameType, rng: &mut ThreadRng) -> GameMove {
    let random_value = rng.gen_range(1..=(if *game_type != GameType::Normal { 5 } else { 3 }));
    match random_value {
        1 => GameMove::ROCK,
        2 => GameMove::PAPER,
        3 => GameMove::SCISSORS,
        4 => {
            if *game_type == GameType::SpockLizard {
                GameMove::SPOCK
            } else {
                GameMove::FIRE
            }
        }
        _ => {
            if *game_type == GameType::SpockLizard {
                GameMove::LIZARD
            } else {
                GameMove::WATER
            }
        }
    }
}

fn write_game_statistics(term: &Term, wins: &u32, loses: &u32, ties: &u32) {
    write_empty_line(term);
    term.write_line(&format!("{}", style("Game statistics:").bold())).unwrap();
    term.write_line(&format!("Total games: {0}, wins: {1}, loses: {2}, ties: {3}",
                             style(wins + loses + ties).cyan().bold(),
                             style(wins).cyan().bold(),
                             style(loses).cyan().bold(),
                             style(ties).cyan().bold())).unwrap();
}

fn do_game() -> io::Result<()> {
    let term = Term::stdout();
    term.set_title("rock_paper_scissors");
    term.clear_screen()?;

    write_overview(&term);

    let mut game_step = MainMenu;
    let mut game_type = GameType::Normal;
    let mut rng = rand::thread_rng();
    let mut wins: u32 = 0;
    let mut loses: u32 = 0;
    let mut ties: u32 = 0;

    loop {
        match game_step {
            MainMenu => {
                let action = get_main_menu(&term);
                match action {
                    main_action::MainAction::AppQuit => game_step = AppStep::AppClosing,
                    main_action::MainAction::GameStart(x) => {
                        game_step = AppStep::GameInProgress;
                        game_type = x;

                        term.write_line(&format!("You choose: {}", style(&game_type.get_name()).bold())).unwrap();
                        write_empty_line(&term);
                    }
                };
            }
            AppStep::GameInProgress => {
                let player_move = parse_player_move(&term, &game_type);
                if player_move == GameMove::QUIT {
                    game_step = AppStep::GameFinished;
                    continue;
                }
                let computer_move = get_computer_move(&game_type, &mut rng);
                let game_result = player_move.beats_other(&game_type, &computer_move);
                let result_message = match  game_result {
                    GameResult::WIN => {
                        wins += 1;
                        "You win !!!"
                    }
                    GameResult::LOSE => {
                        loses += 1;
                        "You lose !!!"
                    }
                    GameResult::TIE => {
                        ties += 1;
                        "Nobody wins !!!"
                    }
                };
                write_empty_line(&term);
                term.write_line(&format!("You choose {:?}, and computer choose {:?}",
                    style(player_move.get_friendly_name()).bold(),
                    style(computer_move.get_friendly_name()).bold())).unwrap();
                term.write_line(&format!("{}", style(result_message).bold())).unwrap();
                term.write_line(&format!("{}", style(GameMove::get_phrase(&player_move, &computer_move)).bold())).unwrap();
            }
            AppStep::GameFinished => {
                game_step = MainMenu;
                write_game_statistics(&term, &wins, &loses, &ties);
                wins = 0;
                loses = 0;
                ties = 0;
            }
            AppStep::AppClosing => {
                term.write_line("Bye, bye").unwrap();
                break;
            }
        }
    }

    Ok(())
}

fn main() {
    do_game().unwrap();
}
