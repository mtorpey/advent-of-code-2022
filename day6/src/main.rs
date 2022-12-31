use std::fs;

fn main() {
    let content = fs::read_to_string("input").expect("Could not read input file");
    println!("Part 1: {}", first_marker_position(&content, 4));
    println!("Part 2: {}", first_marker_position(&content, 14));
}

fn first_marker_position(content: &str, marker_length: usize) -> usize {
    // Queue of the latest chars that were read, all distinct.
    let mut recent: Vec<char> = vec![];

    // Iterate through characters
    for (pos, c) in content.chars().enumerate() {
        // Has new character has been seen lately?
        match recent.iter().position(|x| *x == c) {
            Some(duplicate) => {
                // Yes: Clear queue up to last appearance
                recent.drain(0..=duplicate);
            }
            None => {
                // No: Check whether queue is long enough to be marker
                if recent.len() + 1 >= marker_length {
                    return pos + 1;
                };
            }
        };
        // Add new character to end of queue
        recent.push(c);
    }

    // No marker found in whole string
    content.len()
}
