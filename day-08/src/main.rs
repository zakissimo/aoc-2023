use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug)]
struct Input<'a> {
    directions: &'a str,
    map: HashMap<&'a str, (&'a str, &'a str)>,
}

fn parse(input: &str) -> Option<Input> {
    let mut lines = input.lines();

    if let Some(directions) = lines.next() {
        let map: HashMap<&str, (&str, &str)> = lines
            .filter(|line| !line.is_empty())
            .map(|line| (&line[0..3], (&line[7..10], &line[12..15])))
            .collect();

        return Some(Input { directions, map });
    }

    None
}

fn part_one(input: &str) -> usize {
    let mut steps = 0;
    if let Some(input) = parse(input) {
        let mut current = "AAA";
        while current != "ZZZ" {
            let dir = input.directions.chars().nth(steps % input.directions.len());
            current = match dir {
                Some('L') => input.map.get(current).expect("Triplet should be in map").0,
                Some('R') => input.map.get(current).expect("Triplet should be in map").1,
                _ => panic!("Invalid direction"),
            };
            steps += 1;
        }
    }
    steps
}

fn part_two(input: &str) -> usize {
    let mut steps = 0;
    if let Some(input) = parse(input) {
        let start_vec = input.map.keys().filter(|s| s.chars().nth(2) == Some('A')).map(|s| *s).collect::<Vec<&str>>();
        println!("{:?}", start_vec);
    }
    steps
}

fn main() -> Result<(), Box<dyn Error>> {

    let sample_one = read_to_string("sample_one")?;
    let sample_two = read_to_string("sample_two")?;
    let sample_three = read_to_string("sample_three")?;
    let input = read_to_string("input")?;

    println!("Part One Sample One: {}", part_one(&sample_one));
    println!("Part One Sample Two: {}", part_one(&sample_two));
    println!("Part One Input Two: {}", part_one(&input));

    println!("Part Two Sample Three: {}", part_two(&sample_three));

    Ok(())
}
