use std::{fs::File, io::BufRead, path::Path};

/// Returns an iterator over the lines of a file.
/// The iterator is backed by a BufferedReader, so the file is not loaded into memory at once.
pub fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}

/// Processes a line, taking a closure with the arguments line number and line content as argument.
/// The closure is called once per line.
pub fn process_file_linewise<P: AsRef<Path>, F: FnMut(usize, String)>(file: P, mut processor: F) {
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
