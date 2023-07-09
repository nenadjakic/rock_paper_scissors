use crate::game_error::GameError;
use crate::game_type::GameType;
use crate::main_action::MainAction::{AppQuit, GameStart};

#[derive(Debug, PartialEq)]
pub enum MainAction {
    AppQuit,
    GameStart(GameType),
}

pub fn get_main_actions_menu() -> String {
    r#"
Choose game type or quit
N|n Rock paper scissors
S|s Rock paper scissors Spock lizard
F|f Rock paper scissors fire water

Q|q Quit
"#
    .to_string()
}

pub fn choose_action(input: char) -> Result<MainAction, GameError> {
    match input {
        'N' | 'n' => Ok(GameStart(GameType::Normal)),
        'S' | 's' => Ok(GameStart(GameType::SpockLizard)),
        'F' | 'f' => Ok(GameStart(GameType::FireWater)),
        'Q' | 'q' => Ok(AppQuit),
        _ => Err(GameError(
            "Unknown character: ".to_string() + input.to_string().as_str() + "! Please try again",
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::game_type::GameType::Normal;
    use super::*;

    #[test]
    fn test_get_main_actions_menu() {
        let actual = get_main_actions_menu();
        assert_eq!(actual.lines().count(), 7);
        assert!(actual.contains("N|n"));
        assert!(actual.contains("S|s"));
        assert!(actual.contains("F|f"));
        assert!(actual.contains("Q|q"));
    }

    #[test]
    fn test_choose_action_valid_input_char() {
        let valid_chars = ['N', 'n', 'S', 's', 'F', 'f', 'Q', 'q'];
        for c in valid_chars {
            assert!(choose_action(c).is_ok());
        }
        assert_eq!(choose_action('N'), Ok(GameStart(Normal)));
    }

    #[test]
    fn test_choose_actoin_invalid_input_char() {
        let actual = choose_action('x');
        assert!(actual.is_err());
    }
}