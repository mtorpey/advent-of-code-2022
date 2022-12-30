use regex::Regex;
use std::fs;

struct Move {
    from: usize,
    to: usize,
    num: u32,
}

fn main() {
    let (crate_stacks, moves) = read_input("input");
    run_crate_mover("Part 1", crate_stacks.clone(), &moves, false);
    run_crate_mover("Part 2", crate_stacks, &moves, true);
}

fn run_crate_mover(header: &str, mut stacks: Vec<Vec<char>>, moves: &Vec<Move>, multi_move: bool) {
    print!("{header}: ");
    for m in moves {
        apply_move(&mut stacks, m, multi_move);
    }
    print_stack_tops(&stacks);
}

fn read_input(filename: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let content = fs::read_to_string(filename).expect("Could not read file");
    let lines: Vec<&str> = content.trim().split('\n').collect();
    let crate_rows = get_crate_rows(&lines);
    let crate_stacks = get_crate_stacks(crate_rows);
    let moves = get_crane_moves(&lines);
    (crate_stacks, moves)
}

fn get_crate_rows(lines: &Vec<&str>) -> Vec<Vec<Option<char>>> {
    let mut crate_rows = vec![];
    for line in lines {
        let crates = crates_from_line(line);
        if crates.len() > 1 {
            crate_rows.push(crates);
        } else {
            break;
        }
    }
    crate_rows
}

fn get_crate_stacks(rows: Vec<Vec<Option<char>>>) -> Vec<Vec<char>> {
    let mut crate_stacks: Vec<Vec<char>> = vec![];
    let nr_stacks = rows[rows.len() - 1].len();
    for _ in 0..nr_stacks {
        crate_stacks.push(vec![]);
    }
    for row in rows.iter().rev() {
        for (pos, entry) in row.iter().enumerate() {
            match entry {
                Some(c) => crate_stacks[pos].push(*c),
                None => (),
            }
        }
    }

    crate_stacks
}

fn crates_from_line(line: &str) -> Vec<Option<char>> {
    let re = Regex::new(r"\[([A-Z])\]").unwrap();
    let mut index = 0;
    let mut result = vec![];
    let mut contains_crates = false;
    while index * 4 + 3 <= line.len() {
        let segment = &line[index * 4..index * 4 + 3];
        if re.is_match(segment) {
            let c = re
                .captures(segment)
                .unwrap()
                .get(1)
                .expect("First match missing")
                .as_str()
                .chars()
                .next()
                .unwrap();
            result.push(Some(c));
            contains_crates = true;
        } else {
            result.push(None)
        }
        index += 1;
    }
    if contains_crates {
        result
    } else {
        vec![]
    }
}

fn _print_stacks(crate_stacks: &Vec<Vec<char>>) {
    for stack in crate_stacks {
        for item in stack {
            print!("{}", item);
        }
        println!();
    }
}

fn print_stack_tops(crate_stacks: &Vec<Vec<char>>) {
    for stack in crate_stacks {
        print!("{}", stack[stack.len() - 1]);
    }
    println!();
}

fn get_crane_moves(lines: &Vec<&str>) -> Vec<Move> {
    let mut moves = vec![];
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in lines {
        if re.is_match(line) {
            // ignore non-matching lines
            let cap = re.captures(line).unwrap();
            moves.push(Move {
                num: cap[1].parse().expect("Couldn't parse number"),
                from: cap[2].parse().expect("Couldn't parse number"),
                to: cap[3].parse().expect("Couldn't parse number"),
            });
        }
    }
    moves
}

fn apply_move(stacks: &mut [Vec<char>], m: &Move, multi_move: bool) {
    // Crates held by the crane
    let mut buffer: Vec<char> = vec![];

    // Pick up crates onto the crane
    for _ in 0..m.num {
        let c = stacks[m.from - 1].pop().expect("Cannot move any more)");
        buffer.push(c);
    }

    // If they can be picked up in one go, it should be FILO
    if multi_move {
        buffer.reverse();
    }

    // Put the crates onto the destination stack
    for c in buffer {
        stacks[m.to - 1].push(c);
    }
}
