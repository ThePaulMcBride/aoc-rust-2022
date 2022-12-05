use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn covert_character_to_priority(character: &str) -> u32 {
    match character {
        "a" => 1,
        "b" => 2,
        "c" => 3,
        "d" => 4,
        "e" => 5,
        "f" => 6,
        "g" => 7,
        "h" => 8,
        "i" => 9,
        "j" => 10,
        "k" => 11,
        "l" => 12,
        "m" => 13,
        "n" => 14,
        "o" => 15,
        "p" => 16,
        "q" => 17,
        "r" => 18,
        "s" => 19,
        "t" => 20,
        "u" => 21,
        "v" => 22,
        "w" => 23,
        "x" => 24,
        "y" => 25,
        "z" => 26,
        "A" => 27,
        "B" => 28,
        "C" => 29,
        "D" => 30,
        "E" => 31,
        "F" => 32,
        "G" => 33,
        "H" => 34,
        "I" => 35,
        "J" => 36,
        "K" => 37,
        "L" => 38,
        "M" => 39,
        "N" => 40,
        "O" => 41,
        "P" => 42,
        "Q" => 43,
        "R" => 44,
        "S" => 45,
        "T" => 46,
        "U" => 47,
        "V" => 48,
        "W" => 49,
        "X" => 50,
        "Y" => 51,
        "Z" => 52,
        _ => 0,
    }
}
fn main() {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    // Part 1
    let mut total = 0;
    for line in lines.iter() {
        let (first_half, second_half) = line.split_at(line.len() / 2);

        for character in first_half.chars() {
            if second_half.contains(character) {
                total += covert_character_to_priority(&character.to_string());
                break;
            }
        }
    }

    println!("Part 1: {}", total);

    // Part 2
    let mut total = 0;
    for line in lines.chunks(3) {
        for character in line[0].chars() {
            if line[1].contains(character) && line[2].contains(character) {
                total += covert_character_to_priority(&character.to_string());
                break;
            }
        }
    }

    println!("Part 2: {}", total);
}
