use std::fs;

fn main() {
    // Get input
    let file_path = "input";
    let contents = fs::read_to_string(file_path).expect("File could not be read");
    let lines = contents.split("\n");

    // Simulate games
    let mut total = 0;
    for line in lines {
        let plays: Vec<&str> = line.split(" ").collect();
        if plays.len() > 1 {
            // Run this game
            let opponent_play: u32 = match plays[0] {
                "A" => 0,  // rock
                "B" => 1,  // paper
                "C" => 2,  // scissors
                _ => {println!("Fail"); 0}
            };
            let (my_play, result_points): (u32, u32) = match plays[1] {
                "X" => ((opponent_play + 2) % 3, 0),  // I lose
                "Y" => (opponent_play, 3),  // Draw
                "Z" => ((opponent_play + 1) % 3, 6),  // I win
                _ => {println!("Fail"); (0, 0)}
            };

            // Add the points to the total
            let points: u32 = result_points + my_play + 1;
            total += points
        }
    }

    // Report result
    println!("{}", total);
}
