use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}
impl Outcome {
    pub fn is_loss(&self) -> bool {
        self == &Self::Loss
    }
    pub fn is_draw(&self) -> bool {
        self == &Self::Draw
    }
    pub fn is_win(&self) -> bool {
        self == &Self::Win
    }
    pub fn score(&self) -> u32 {
        *self as u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
impl Shape {
    /// Plays `self` against `other`, determining whether `self` wins.
    pub fn play_against(self, other: Shape) -> Outcome {
        use Shape::*;
        if self == other {
            return Outcome::Draw;
        }
        match (self, other) {
            (Scissors, Paper) => Outcome::Win,
            (Paper, Rock) => Outcome::Win,
            (Rock, Scissors) => Outcome::Win,
            _ => Outcome::Loss,
        }
    }
    pub fn score(&self) -> u32 {
        *self as u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Round {
    opponent_play: Shape,
    response: Outcome,
}
impl Round {
    pub fn new(opponent_play: Shape, response: Outcome) -> Self {
        Self {
            opponent_play,
            response,
        }
    }

    pub fn calc_score(&self) -> u32 {
        use Shape::*;
        let my_play = match self.response {
            Outcome::Loss => match self.opponent_play {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            },
            Outcome::Draw => self.opponent_play,
            Outcome::Win => match self.opponent_play {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
            },
        };
        my_play.play_against(self.opponent_play).score() + my_play.score()
    }
}
#[derive(Debug)]
pub enum RoundParseError {
    MissingLetter(usize),
    UnexpectedInput(String),
}
impl FromStr for Round {
    type Err = RoundParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ').filter(|s| !s.is_empty());
        let opponent_play = match iter.next() {
            Some("A") => Shape::Rock,
            Some("B") => Shape::Paper,
            Some("C") => Shape::Scissors,
            Some(other) => return Err(RoundParseError::UnexpectedInput(other.into())),
            None => return Err(RoundParseError::MissingLetter(0)),
        };
        let response = match iter.next() {
            Some("X") => Outcome::Loss,
            Some("Y") => Outcome::Draw,
            Some("Z") => Outcome::Win,
            Some(other) => return Err(RoundParseError::UnexpectedInput(other.into())),
            None => return Err(RoundParseError::MissingLetter(1)),
        };
        Ok(Self {
            opponent_play,
            response,
        })
    }
}

pub struct Rounds(Vec<Round>);
impl Rounds {
    pub fn iter(&self) -> std::slice::Iter<Round> {
        self.0.iter()
    }
    // evaluate all rounds and sum them together
    pub fn evaluate_all(&self) -> u32 {
        self.iter().map(|round| round.calc_score()).sum()
    }
}
impl FromStr for Rounds {
    type Err = RoundParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = parse_input(s)?;
        Ok(Self(vec))
    }
}
fn parse_input(input: &str) -> Result<Vec<Round>, RoundParseError> {
    input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| line.parse::<Round>())
        // ye olde collecting into a Result :3
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Outcome::*;
    use Shape::*;

    const EXAMPLE_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn basic_rules() {
        assert!(Rock.play_against(Rock).is_draw());
        assert!(Rock.play_against(Paper).is_loss());
        assert!(Rock.play_against(Scissors).is_win());

        assert!(Paper.play_against(Rock).is_win());
        assert!(Paper.play_against(Paper).is_draw());
        assert!(Paper.play_against(Scissors).is_loss());

        assert!(Scissors.play_against(Rock).is_loss());
        assert!(Scissors.play_against(Paper).is_win());
        assert!(Scissors.play_against(Scissors).is_draw());
    }

    #[test]
    fn round_evaluation() {
        assert_eq!(Round::new(Rock, Draw).calc_score(), 4);
        assert_eq!(Round::new(Paper, Loss).calc_score(), 1);
        assert_eq!(Round::new(Scissors, Win).calc_score(), 7);

        // finally, the expected result for the example input is 15
        assert_eq!(EXAMPLE_INPUT.parse::<Rounds>().unwrap().evaluate_all(), 12);
    }

    #[test]
    fn round_parsing() {
        assert_eq!("A Y".parse::<Round>().unwrap(), Round::new(Rock, Draw));
        assert_eq!("B X".parse::<Round>().unwrap(), Round::new(Paper, Loss));
        assert_eq!("C Z".parse::<Round>().unwrap(), Round::new(Scissors, Win));

        // These asserts are a bit complicated because of how matches!
        // doesn't really work with strings
        let error = "a blub".parse::<Round>().unwrap_err();
        let RoundParseError::UnexpectedInput(s) = error else {
            panic!("unexpected kind of error: {:?}", error)
        };
        assert_eq!(s, "a");
        let error = "B blub".parse::<Round>().unwrap_err();
        let RoundParseError::UnexpectedInput(s) = error else {
            panic!("unexpected kind of error: {:?}", error)
        };
        assert_eq!(s, "blub");

        // these are much better
        assert!(matches!(
            "".parse::<Round>(),
            Err(RoundParseError::MissingLetter(0))
        ));
        assert!(matches!(
            "B".parse::<Round>(),
            Err(RoundParseError::MissingLetter(1))
        ));

        // make sure the entire example input gets parsed correctly
        let rounds = EXAMPLE_INPUT
            .parse::<Rounds>()
            .expect("example input should parse");
        let mut rounds_iter = rounds.iter();
        let round1 = rounds_iter.next().expect("should have first round");
        assert_eq!(round1, &Round::new(Rock, Draw));
        let round2 = rounds_iter.next().expect("should have second round");
        assert_eq!(round2, &Round::new(Paper, Loss));
        let round3 = rounds_iter.next().expect("should have third round");
        assert_eq!(round3, &Round::new(Scissors, Win));
        assert_eq!(rounds_iter.next(), None, "should only have three rounds");
    }
}
