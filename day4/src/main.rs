use std::fs;

type Range = (u32, u32);

fn main() {
    // Get input
    let filename = "input";
    let content = fs::read_to_string(filename).expect("Failed to read file '{input}'");
    let lines: Vec<&str> = content.trim().split('\n').collect();

    // Process into a data structure
    let assignments = read_assignments(lines).expect("Bad file");

    // Calculate problem solutions
    println!(
        "Part 1: {}",
        nr_assignments_satisfying(&assignments, contains)
    );
    println!(
        "Part 2: {}",
        nr_assignments_satisfying(&assignments, overlaps)
    );
}

fn read_assignments(lines: Vec<&str>) -> Result<Vec<(Range, Range)>, &str> {
    let mut assignments = vec![];
    for line in lines {
        assignments.push(match line.split(',').collect::<Vec<&str>>().as_slice() {
            [elf1, elf2] => (read_range(elf1), read_range(elf2)),
            _ => return Err("Line does not match expected form"),
        });
    }
    Ok(assignments)
}

fn read_range(range: &str) -> Range {
    match range.split('-').collect::<Vec<&str>>().as_slice() {
        [min, max] => (
            min.parse().expect("Bad integer"),
            max.parse().expect("Bad integer"),
        ),
        _ => (0, 0),
    }
}

fn nr_assignments_satisfying(
    assignments: &Vec<(Range, Range)>,
    check: fn(&Range, &Range) -> bool,
) -> u32 {
    let mut count = 0;
    for (elf1, elf2) in assignments {
        if check(elf1, elf2) || check(elf2, elf1) {
            count += 1;
        }
    }
    count
}

fn contains((min1, max1): &Range, (min2, max2): &Range) -> bool {
    min1 <= min2 && max1 >= max2
}

fn overlaps((min1, max1): &Range, (min2, max2): &Range) -> bool {
    min2 <= max1 && min1 <= max2
}
