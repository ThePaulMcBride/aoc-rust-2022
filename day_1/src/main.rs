use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./src/input.txt").unwrap();
    let reader = BufReader::new(file);

    // Part 1
    let mut totals: Vec<u32> = Vec::new();
    let mut current_total: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            totals.push(current_total);
            current_total = 0;
        } else {
            let number = line.parse::<u32>().unwrap();
            current_total = current_total + number;
        }
    }

    let mut largest: u32 = 0;
    for &total in totals.iter() {
        if total > largest {
            largest = total;
        }
    }

    println!("Largest: {largest}");

    // Part 2
    let mut largest_3: Vec<u32> = Vec::new();
    for &total in totals.iter() {
        if largest_3.len() < 3 {
            largest_3.push(total);
        } else {
            let mut smallest = largest_3[0];
            let mut smallest_index = 0;
            for (index, &number) in largest_3.iter().enumerate() {
                if number < smallest {
                    smallest = number;
                    smallest_index = index;
                }
            }

            if total > smallest {
                largest_3[smallest_index] = total;
            }
        }
    }

    let mut largest_3_total: u32 = 0;
    for &number in largest_3.iter() {
        largest_3_total = largest_3_total + number;
    }

    println!("Largest 3: {largest_3_total}");
}
