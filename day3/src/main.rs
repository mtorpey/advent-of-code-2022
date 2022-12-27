use std::fs;

fn main() {
    // Get text
    let content = fs::read_to_string("input")
        .expect("Could not read input file");
    let lines: Vec<&str> = content.trim().split("\n").collect();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(lines: &Vec<&str>) -> u32 {
    // Read through lines
    let mut sum: u32 = 0;
    for line in lines {
        let len = line.len();
        // Split into two compartments
        //println!("Line: {}", line);
        let part1 = &line[0..len / 2];
        let part2 = &line[len / 2..];
        // Find the problem
        let item = compare_bag_compartments(part1, part2)
            .expect("No error found");
        sum += item_priority(item);
    }
    return sum;
}

fn compare_bag_compartments(part1: &str, part2: &str) -> Result<char, &'static str> {
    for c in part1.chars() {
        if part2.contains(c) {
            return Ok(c);
        }
    }
    return Err("Not found");
}

fn part2(lines: &Vec<&str>) -> u32 {
    let mut sum = 0;
    for chunk in lines.chunks(3) {
        sum += match chunk {
            [bag1, bag2, bag3] => {
                let item = common_item(bag1, bag2, bag3)
                    .expect("Not a multiple of 3 lines");
                item_priority(item)
            },
            _ => 0
        }
    }
    return sum;
}

fn common_item(bag1: &str, bag2: &str, bag3: &str) -> Result<char, &'static str> {
    for c in bag1.chars() {
        if bag2.contains(c) && bag3.contains(c) {
            return Ok(c);
        }
    }
    return Err("No common item");
}

fn item_priority(item: char) -> u32 {
    let val = item as u32;
    let priority = if item >= 'a' && item <= 'z' {
        val - 96
    } else if item >= 'A' && item <= 'Z' {
        val - 64 + 26
    } else {
        println!("Invalid item!");
        0
    };
    //println!("{} has priority {}", item, priority);
    return priority;
}
