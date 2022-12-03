use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

#[derive(Debug)]
struct Round {
    opponent: Rps,
    you: Rps,
}

#[derive(Debug)]
struct OutcomeRound {
    opponent: Rps,
    outcome: Outcome,
}

impl FromStr for Round {
    type Err = String;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let opponent = match &str[0..1] {
            "A" => Ok(Rps::Rock),
            "B" => Ok(Rps::Paper),
            "C" => Ok(Rps::Scissors),

            other => Err(format!("Unknown letter {other}")),
        }?;
        let you = match &str[2..3] {
            "X" => Ok(Rps::Rock),
            "Y" => Ok(Rps::Paper),
            "Z" => Ok(Rps::Scissors),

            other => Err(format!("Unknown letter {other}")),
        }?;

        Ok(Round { opponent, you })
    }
}

impl FromStr for OutcomeRound {
    type Err = String;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let opponent = match &str[0..1] {
            "A" => Ok(Rps::Rock),
            "B" => Ok(Rps::Paper),
            "C" => Ok(Rps::Scissors),

            other => Err(format!("Unknown letter {other}")),
        }?;
        let you = match &str[2..3] {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),

            other => Err(format!("Unknown letter {other}")),
        }?;

        Ok(OutcomeRound {
            opponent,
            outcome: you,
        })
    }
}

impl Round {
    fn score(&self) -> i32 {
        let win = match self {
            Round {
                opponent: Rps::Rock,
                you: Rps::Paper,
            } => true,
            Round {
                opponent: Rps::Paper,
                you: Rps::Scissors,
            } => true,
            Round {
                opponent: Rps::Scissors,
                you: Rps::Rock,
            } => true,
            _ => false,
        };
        let draw = self.opponent == self.you;

        let choice_score = match self.you {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        };

        let round_score = if win {
            6
        } else {
            if draw {
                3
            } else {
                0
            }
        };

        choice_score + round_score
    }
}

impl OutcomeRound {
    fn to_round(&self) -> Round {
        let opponent = self.opponent;
        let you = match (&self.opponent, &self.outcome) {
            (Rps::Rock, Outcome::Lose) => Rps::Scissors,
            (Rps::Rock, Outcome::Win) => Rps::Paper,
            (Rps::Paper, Outcome::Lose) => Rps::Rock,
            (Rps::Paper, Outcome::Win) => Rps::Scissors,
            (Rps::Scissors, Outcome::Lose) => Rps::Paper,
            (Rps::Scissors, Outcome::Win) => Rps::Rock,
            (other, Outcome::Draw) => *other,
        };
        Round { opponent, you }
    }
}

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let parsed: Result<Vec<Round>, _> = lines.iter().map(|s| Round::from_str(s)).collect();

    let part1: i32 = parsed?.iter().map(|r| r.score()).sum();

    println!("Part 1 {}", part1);

    let parsed: Result<Vec<OutcomeRound>, _> =
        lines.iter().map(|s| OutcomeRound::from_str(s)).collect();

    let part2: i32 = parsed?
        .iter()
        .map(|r| r.to_round())
        .map(|r| r.score())
        .sum();

    println!("Part 2 {}", part2);

    Ok(())
}
