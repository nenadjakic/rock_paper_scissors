#[derive(Debug, PartialEq)]
pub enum GameType {
    Normal,
    SpockLizard,
    FireWater,
}

impl GameType {
    pub fn get_move_menu(&self) -> String {
        let shared_text = r#"
Enter your move:

R|r Rock
P|p Paper
S|s Scissors
"#
        .to_string();

        (match self {
            GameType::Normal => shared_text,
            GameType::SpockLizard => {
                shared_text
                    + r#"O|o Spock
L|l Lizard
"#
            }
            GameType::FireWater => {
                shared_text
                    + r#"F|f Fire
W|w Water
"#
            }
        }) + r#"
Q|q Finish game"#
    }

    pub fn get_name(&self) -> String {
        match self {
            GameType::Normal => "Normal game".to_string(),
            GameType::SpockLizard => "Spock-lizard variation".to_string(),
            GameType::FireWater => "Fire-water variation".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game_type::GameType;

    #[test]
    fn test_get_move_menu() {
        let actual_normal = GameType::Normal.get_move_menu();
        let actual_spock = GameType:: SpockLizard.get_move_menu();
        let actual_fire = GameType:: FireWater.get_move_menu();

        assert_eq!(actual_normal.lines().count(), 8);
        test_normal_move_menu(actual_normal.as_str());

        assert_eq!(actual_spock.lines().count(), 10);
        test_normal_move_menu(actual_spock.as_str());
        assert!(actual_spock.contains("O|o"));
        assert!(actual_spock.contains("L|l"));

        assert_eq!(actual_fire.lines().count(), 10);
        test_normal_move_menu(actual_fire.as_str());
        assert!(actual_fire.contains("F|f"));
        assert!(actual_fire.contains("W|w"));
    }

    #[test]
    fn test_get_name() {
        assert_eq!(GameType::Normal.get_name(), "Normal game");
        assert_eq!(GameType::SpockLizard.get_name(), "Spock-lizard variation");
        assert_eq!(GameType::FireWater.get_name(), "Fire-water variation");
    }

    fn test_normal_move_menu(actual: &str) {
        assert!(actual.contains("Q|q"));
        assert!(actual.contains("R|r"));
        assert!(actual.contains("P|p"));
        assert!(actual.contains("S|s"));
    }
}