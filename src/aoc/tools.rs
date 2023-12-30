pub fn string_to_digit(s: &str) -> Option<i32> {
    match s {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "0" => Some(0),
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        "4" => Some(4),
        "5" => Some(5),
        "6" => Some(6),
        "7" => Some(7),
        "8" => Some(8),
        "9" => Some(9),
        _ => None,
    }
}

/// Parses an input string by splitting at a new line, stripping whitespace characters and returning it as a list
pub fn read_lines(input: &str) -> Vec<String> {
    input
        .trim()
        .split("\n")
        .map(|s| s.trim().to_string())
        .collect()
}


/// Parses and input string by splitting at a double new line, then splits each line at a single new line
/// Optionally a block can contain a header, that is a string that if matched in the first line, it will be ignored
pub fn read_blocks(input: &str, header: Option<&str>) -> Vec<Vec<String>> {
    let mut blocks = Vec::new();
    let mut block = Vec::new();
    for raw_block in input.trim().split("\n\n") {
        let mut lines = raw_block.trim()
            .split("\n")
            .map(|s| s.trim().to_string());

        let mut first = true;
        for line in lines.by_ref() {
            if let Some(header) = header {
                if first {
                    first = false;
                    if line.contains(header) {
                        continue;
                    }
                }
            }
            block.push(line.to_string());
        }

        blocks.push(block.clone());
        block.clear();
    }
    blocks
}

/// Parses and input string by splitting each line at whitespaces.
/// Assuming each line has the same amount of columns.
pub fn read_columns(input: &str) -> Vec<Vec<String>> {
    let matrix: Vec<Vec<String>> = read_lines(input).iter()
        .map(|r| r.split_whitespace()
            .map(|s| s.to_string()).collect())
        .collect();

    (0..matrix[0].len())
        .map(|i| matrix.iter()
            .map(|inner| inner[i].clone())
            .collect())
        .collect()
}

/// Given an input string and a predicate, returns the first character matching the predicate
pub fn find_char(input: &str, predicate: fn(char) -> bool) -> Option<char> {
    input
        .chars()
        .find(|c| predicate(*c))
}

/// Given an input string and a predicate, returns the first character matching the predicate
pub fn rfind_char(input: &str, predicate: fn(char) -> bool) -> Option<char> {
    input
        .chars()
        .rev()
        .find(|c| predicate(*c))
}


/// Given an input string and a list of substrings returns the first occurrence of any of the substrings.
/// The value with the lowest start index within the input is returned.
/// Example: find_substrings("1onetwo", vec!["two", "one", "1"]) returns "1"
pub fn find_substrings(input: &str, substrings: &[&str]) -> Option<String> {
    substrings
        .iter()
        .filter_map(|s| input.find(s).map(|i| (i, s)))
        .min_by_key(|(i, _)| *i)
        .map(|(_, s)| s.to_string())
}

/// Given an input string and a list of substrings returns the first occurrence of any of the substrings.
/// The value with the highest start index within the input is returned.
/// Example: rfind_substrings("1onetwo", vec!["1", "one", "two"]) returns "two"
pub fn rfind_substrings(input: &str, substrings: &[&str]) -> Option<String> {
    substrings
        .iter()
        .filter_map(|s| input.rfind(s).map(|i| (i, s)))
        .max_by_key(|(i, _)| *i)
        .map(|(_, s)| s.to_string())
}