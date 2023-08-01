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