use std::cmp::Reverse;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

pub fn run(p: PathBuf, word: Option<String>) -> Result<(), Box<dyn Error>> {
    let filename = PathBuf::from(p);
    let word_to_search = word;

    let word_counts = count_words(&filename)?;

    let counts = frequency(word_counts);

    if let Some(word) = word_to_search {
        let mut count = 0;
        for (w, c) in &counts {
            if w == &word {
                count = *c;
                break;
            }
        }
        println!("{}: {}", word, count);
    } else {
        for (word, count) in counts {
            println!("{}: {}", word, count);
        }
    }
    Ok(())
}

pub fn count_words(filename: &PathBuf) -> io::Result<HashMap<String, u32>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut word_counts = HashMap::new();
    
    let mut lines = reader.lines();

    // Check if the file is empty
    if lines.next().is_none() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "empty file"));
    }

    for line_result in lines {
        let line = line_result?;
        for word in line.split_whitespace() {
            let word = word
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>();

            *word_counts.entry(word.to_lowercase()).or_insert(0) += 1;
        }
    }
    Ok(word_counts)
}

fn frequency(word_counts: HashMap<String, u32>) -> Vec<(String, u32)> {
    let mut counts: Vec<(String, u32)> = word_counts.into_iter().collect();
    counts.sort_by_key(|&(_, count)| Reverse(count));
    counts.into_iter().collect()
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn file_not_found_test() {
        let file_path = "test_files/nonexistent.txt";
        let result = run(PathBuf::from(file_path), None);
        assert!(result.is_err());
    }
}
