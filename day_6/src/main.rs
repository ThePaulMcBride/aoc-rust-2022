use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    let first_line = reader.lines().next().unwrap().unwrap();
    let input = String::from(first_line);

    // Part 1
    let mut cursor = 0;
    let mut found = false;
    while !found {
        let chars: Vec<_> = input.chars().collect();
        let slice = &chars[cursor..cursor + 4];
        let mut unique = true;

        for i in 0..slice.len() {
            for j in 0..slice.len() {
                if i != j && slice[i] == slice[j] {
                    unique = false;
                }
            }
        }

        if unique {
            found = true;
        } else {
            cursor += 1;
        }
    }

    println!("Part 1: {}", cursor + 4);

    // Part 2
    let mut cursor = 0;
    let mut found = false;
    while !found {
        let chars: Vec<_> = input.chars().collect();
        let slice = &chars[cursor..cursor + 14];
        let mut unique = true;

        for i in 0..slice.len() {
            for j in 0..slice.len() {
                if i != j && slice[i] == slice[j] {
                    unique = false;
                }
            }
        }

        if unique {
            found = true;
        } else {
            cursor += 1;
        }
    }

    println!("Part 2: {}", cursor + 14);
}
