use std::{fs::File, io::BufRead};

#[derive(Debug)]
struct Assignment {
    start: u32,
    end: u32,
}
impl Assignment {
    fn new(input: String) -> Self {
        let mut split = input.split('-');
        let start = split.next().unwrap().parse().unwrap();
        let end = split.next().unwrap().parse().unwrap();
        Self { start, end }
    }

    fn contains(&self, assignment: &Assignment) -> bool {
        self.start <= assignment.start && assignment.end <= self.end
    }

    fn overlaps(&self, assignment: &Assignment) -> bool {
        self.start <= assignment.start && assignment.start <= self.end
            || self.start <= assignment.end && assignment.end <= self.end
    }
}
fn main() {
    let file = File::open("src/input.txt").unwrap();
    let mut lines: Vec<String> = Vec::new();
    for line in std::io::BufReader::new(file).lines() {
        lines.push(line.unwrap());
    }

    // Part 1
    let mut total = 0;
    for line in lines.iter() {
        let assignments = line
            .split(",")
            .map(|s| Assignment::new(s.to_string()))
            .collect::<Vec<Assignment>>();

        let first_assignment = &assignments[0];
        let second_assignment = &assignments[1];

        if first_assignment.contains(second_assignment)
            || second_assignment.contains(first_assignment)
        {
            total += 1;
        }
    }

    println!("Part 1: {total}");

    // Part 2
    let mut total = 0;
    for line in lines.iter() {
        let assignments = line
            .split(",")
            .map(|s| Assignment::new(s.to_string()))
            .collect::<Vec<Assignment>>();

        let first_assignment = &assignments[0];
        let second_assignment = &assignments[1];

        if first_assignment.overlaps(second_assignment)
            || second_assignment.overlaps(first_assignment)
        {
            total += 1;
        }
    }

    println!("Part 2: {total}");
}
