use util::process_file_linewise;

fn main() {
    println!("Score: {}", compute_scores());

    println!("Score2: {}", compute_scores_2());
}

fn compute_scores() -> usize {
    let mut total = 0;

    process_file_linewise("./input", |line_no, line| {
        let inputs = extract_chars(line_no, line);

        let own = Shape::try_from(inputs[0]).unwrap_or_else(|_| {
            panic!(
                "Could not create shape from char {} in line {line_no}",
                inputs[0]
            )
        });
        let other = Shape::try_from(inputs[1]).unwrap_or_else(|_| {
            panic!(
                "Could not create shape from char {} in line {line_no}",
                inputs[1]
            )
        });

        let score = compute_score(own, other);

        total += score;
    });

    total
}

fn compute_scores_2() -> usize {
    let mut total = 0;

    process_file_linewise("./input", |line_no, line| {
        let inputs = extract_chars(line_no, line);

        let own = Shape::try_from(inputs[0]).unwrap_or_else(|_| {
            panic!(
                "Could not create shape from char {} in line {line_no}",
                inputs[0]
            )
        });
        let other = RoundResult::try_from(inputs[1]).unwrap_or_else(|_| {
            panic!(
                "Could not create round result from char {} in line {line_no}",
                inputs[1]
            )
        });

        let score = compute_score_2(own, other);

        total += score;
    });

    total
}

fn extract_chars(line_no: usize, line: String) -> Vec<char> {
    let inputs: Vec<char> = line
        .split(' ')
        .map(|s| {
            s.chars()
                .next()
                .unwrap_or_else(|| panic!("Char in line {line_no} missing, line content: {line}"))
        })
        .collect();

    if inputs.len() != 2 {
        panic!("Line {line_no} did not contain two symbols, line content: {line}!");
    }

    inputs
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn own_score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn compare(&self, other: &Self) -> RoundResult {
        match (self, other) {
            (Shape::Rock, Shape::Rock) => RoundResult::Draw,
            (Shape::Rock, Shape::Paper) => RoundResult::Loose,
            (Shape::Rock, Shape::Scissors) => RoundResult::Win,
            (Shape::Paper, Shape::Rock) => RoundResult::Win,
            (Shape::Paper, Shape::Paper) => RoundResult::Draw,
            (Shape::Paper, Shape::Scissors) => RoundResult::Loose,
            (Shape::Scissors, Shape::Rock) => RoundResult::Loose,
            (Shape::Scissors, Shape::Paper) => RoundResult::Win,
            (Shape::Scissors, Shape::Scissors) => RoundResult::Draw,
        }
    }
}

impl TryFrom<char> for Shape {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Rock),
            'B' => Ok(Self::Paper),
            'C' => Ok(Self::Scissors),
            'X' => Ok(Self::Rock),
            'Y' => Ok(Self::Paper),
            'Z' => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

enum RoundResult {
    Loose,
    Draw,
    Win,
}

impl RoundResult {
    fn shape_for_result(&self, opponent_shape: Shape) -> Shape {
        match (self, opponent_shape) {
            (RoundResult::Loose, Shape::Rock) => Shape::Scissors,
            (RoundResult::Loose, Shape::Paper) => Shape::Rock,
            (RoundResult::Loose, Shape::Scissors) => Shape::Paper,
            (RoundResult::Draw, Shape::Rock) => Shape::Rock,
            (RoundResult::Draw, Shape::Paper) => Shape::Paper,
            (RoundResult::Draw, Shape::Scissors) => Shape::Scissors,
            (RoundResult::Win, Shape::Rock) => Shape::Paper,
            (RoundResult::Win, Shape::Paper) => Shape::Scissors,
            (RoundResult::Win, Shape::Scissors) => Shape::Rock,
        }
    }

    fn score(&self) -> usize {
        match self {
            RoundResult::Loose => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6,
        }
    }
}

impl TryFrom<char> for RoundResult {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Loose),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

fn compute_score(opponent: Shape, own: Shape) -> usize {
    let mut score = 0;

    score += own.own_score();
    score += compute_comparison_from_result(own, opponent);

    score
}

fn compute_comparison_from_result(own: Shape, opponent: Shape) -> usize {
    own.compare(&opponent).score()
}

fn compute_score_2(opponent: Shape, result: RoundResult) -> usize {
    let own_shape = result.shape_for_result(opponent);
    own_shape.own_score() + result.score()
}
