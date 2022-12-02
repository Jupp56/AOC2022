use std::path::Path;

use util::process_file_linewise;

fn main() {
    println!("{}", count_calories("./input"));
}

#[derive(Debug)]
struct Leaderboard {
    board: [u64; 3],
}

impl Leaderboard {
    pub fn new() -> Self {
        Self { board: [0; 3] }
    }

    /// Updates the leaderboard with the new value, if it is higher than any of the leaderboard's values.
    pub fn update(&mut self, new_value: u64) {
        for leader in &mut self.board {
            if *leader < new_value {
                let old_leader = *leader;
                *leader = new_value;

                self.update(old_leader);

                break;
            }
        }
    }

    pub fn sum(&self) -> u64 {
        self.board.into_iter().sum()
    }
}

fn count_calories<P: AsRef<Path>>(file: P) -> u64 {
    let mut leaderboard = Leaderboard::new();

    let mut current_tally = 0;

    process_file_linewise(file, |line, content| {
        if content.trim().is_empty() {
            leaderboard.update(current_tally);
            current_tally = 0;
            return;
        }

        let int = content.as_str().parse::<u64>();
        match int {
            Ok(int) => {
                current_tally += int;
            }
            Err(error) => {
                println!(
                    "Error: Could not parse line {line} with content {content} because of error {:#?}",
                     error
                );
            }
        }
    });

    println!("Leaderboard: {:#?}", leaderboard);

    leaderboard.sum()
}
