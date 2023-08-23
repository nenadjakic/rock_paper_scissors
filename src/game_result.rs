#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    pub fn get_friendly_name(&self) -> String {
        match self {
            GameResult::Win => String::from("You win !!!"),
            GameResult::Lose => String::from("You lose !!!"),
            GameResult::Draw => String::from("Draw !!!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game_result::GameResult::{Draw, Lose, Win};

    #[test]
    fn test_get_friendly_name() {
        assert_eq!(Win.get_friendly_name(), String::from("You win !!!"));
        assert_eq!(Lose.get_friendly_name(), String::from("You lose !!!"));
        assert_eq!(Draw.get_friendly_name(), String::from("Draw !!!"));
    }
}
