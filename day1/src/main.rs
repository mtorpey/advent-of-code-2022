use std::fs;

fn main() {
    // Get input
    let file_path = "input";
    let contents = fs::read_to_string(file_path).expect("File could not be read");
    let lines = contents.split("\n");

    // Setup memory
    let mut totals: Vec<u32> = Vec::new();
    let mut current = 0;

    // Read through the content
    for line in lines {
        if line == "" {
            // Finished this elf
            totals.push(current);
            current = 0;
        } else {
            let num: u32 = match line.trim().parse() {
                Ok(n) => n,
                Err(e) => {println!("ERROR: {}", e); 0}
            };
            current += num;
        }
    }

    // Result
    totals.sort();
    let nr_elves = totals.len();
    let total: u32 = totals[nr_elves-3..].iter().sum();
    println!("{}", total);
}
