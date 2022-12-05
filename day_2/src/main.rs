use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    // Part 1
    let mut total_score = 0;
    for line in lines.iter() {
        let game = line.split_once(" ").unwrap();

        let score = match game {
            ("A", "X") => 1 + 3,
            ("A", "Y") => 2 + 6,
            ("A", "Z") => 3 + 0,
            ("B", "X") => 1 + 0,
            ("B", "Y") => 2 + 3,
            ("B", "Z") => 3 + 6,
            ("C", "X") => 1 + 6,
            ("C", "Y") => 2 + 0,
            ("C", "Z") => 3 + 3,
            _ => 0,
        };

        total_score += score;
    }

    println!("Part 1: {total_score}");

    // Part 2
    let mut total_score = 0;
    for line in lines.iter() {
        let game = line.split_once(" ").unwrap();

        let score = match game {
            ("A", "X") => 0 + 3,
            ("A", "Y") => 3 + 1,
            ("A", "Z") => 6 + 2,
            ("B", "X") => 0 + 1,
            ("B", "Y") => 3 + 2,
            ("B", "Z") => 6 + 3,
            ("C", "X") => 0 + 2,
            ("C", "Y") => 3 + 3,
            ("C", "Z") => 6 + 1,
            _ => 0,
        };

        total_score += score;
    }

    println!("Part 2: {total_score}");
}
