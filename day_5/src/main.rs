use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

struct Ship {
    crates: HashMap<u32, Vec<String>>,
}
impl Ship {
    fn new(setup: &Vec<String>) -> Self {
        let mut ship = Ship {
            crates: HashMap::new(),
        };

        for (index, line) in setup.iter().rev().enumerate() {
            let chunks_ = line.chars();
            let chunks_ = chunks_.collect::<Vec<char>>();
            let chunks = chunks_
                .chunks(4)
                .map(|chunk| chunk.iter().collect::<String>());

            for (stack_index, chunk) in chunks.enumerate() {
                let stack = (stack_index as u32) + 1;
                if index == 0 {
                    let chunk = chunk.trim();
                    let stack = chunk.parse::<u32>().unwrap();
                    ship.add_column(stack);
                }
                if index != 0 {
                    // strip square brackets and spaces
                    let chunk = chunk.trim();
                    if chunk != "" {
                        let chunk = chunk.trim_start_matches('[').trim_end_matches(']');
                        ship.add_crate(stack, chunk.to_string());
                    }
                }
            }
        }

        ship
    }
    fn add_column(&mut self, column: u32) {
        self.crates.entry(column).or_insert(Vec::new());
    }
    fn add_crate(&mut self, stack: u32, item: String) {
        self.crates.entry(stack).or_insert(Vec::new()).push(item);
    }
    fn move_crate(&mut self, from: u32, to: u32) {
        let item = self.crates.get_mut(&from).unwrap().pop().unwrap();
        self.crates.get_mut(&to).unwrap().push(item);
    }
    fn move_crates(&mut self, from: u32, to: u32, count: u32) {
        let index = self.crates.get_mut(&from).unwrap().len() - count as usize;
        let mut items = self
            .crates
            .get_mut(&from)
            .unwrap()
            .drain(index..)
            .collect::<Vec<_>>();
        self.crates.get_mut(&to).unwrap().append(&mut items);
    }
    fn get_top_crates(&self) -> String {
        let mut top_items = String::from("");
        for stack in 1..=self.crates.len() {
            let stack = stack as u32;
            let top_item = self.crates.get(&stack).unwrap().last().unwrap();
            top_items.push_str(top_item);
        }

        top_items
    }
}

fn main() {
    let file = File::open("src/input.txt").unwrap();
    let mut lines: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        lines.push(line.unwrap());
    }
    let mut split = lines.split(|line| line.is_empty());
    let setup_array = split.next().unwrap();
    let setup = setup_array.to_vec();
    let instructions_array = split.next().unwrap();
    let instructions = instructions_array.to_vec();

    // Part 1
    let mut ship = Ship::new(&setup);
    for line in instructions.iter() {
        let split = line.split_whitespace();
        let mut numbers: Vec<u32> = Vec::new();
        for chunks in split {
            let chunks = chunks.trim();
            match chunks.parse::<u32>() {
                Ok(number) => numbers.push(number),
                Err(_) => (),
            }
        }

        let times = numbers[0];
        let from = numbers[1];
        let to = numbers[2];

        for _ in 0..times {
            ship.move_crate(from, to);
        }
    }

    let top_crates = ship.get_top_crates();
    println!("Part 1: {top_crates}");

    // Part 2
    let mut ship = Ship::new(&setup);
    for line in instructions.iter() {
        let split = line.split_whitespace();
        let mut numbers: Vec<u32> = Vec::new();
        for chunks in split {
            // parse numbers
            let chunks = chunks.trim();
            match chunks.parse::<u32>() {
                Ok(number) => numbers.push(number),
                Err(_) => (),
            }
        }

        let times = numbers[0];
        let from = numbers[1];
        let to = numbers[2];

        ship.move_crates(from, to, times);
    }

    let top_crates = ship.get_top_crates();
    println!("Part 2: {top_crates}");
}
