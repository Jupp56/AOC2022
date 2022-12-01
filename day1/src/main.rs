use std::{fs::File, io::BufRead, path::Path};

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

    process_lines(file, |line, content| {
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

/// Processes a line, taking a closure with arguments line number and line content as argument.
fn process_lines<P: AsRef<Path>, F: FnMut(usize, String)>(file: P, mut processor: F) {
    let lines = match read_lines(&file) {
        Ok(lines) => lines,
        Err(error) => {
            panic!("Could not open file {:?}, Error: {error}!", file.as_ref());
        }
    };

    for (line_number, line) in lines.enumerate() {
        match line {
            Ok(content) => {
                processor(line_number, content);
            }
            Err(error) => {
                println!("Could not read line {line_number}, error: {error}");
            }
        }
    }
}

fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}
