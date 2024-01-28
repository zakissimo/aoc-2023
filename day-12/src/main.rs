use anyhow::Result;
use std::fs::read_to_string;

fn parse(input: &str) -> Vec<(&str, Vec<usize>)> {
    input
        .lines()
        .map(|row| {
            if let Some((left, right)) = row.split_once(' ') {
                (
                    left,
                    right
                        .split(',')
                        .map(|e| e.parse::<usize>().expect("Input should be digits only"))
                        .collect(),
                )
            } else {
                panic!("Input isn't formatted correctly!");
            }
        })
        .collect()
}

fn part_one(input: &str) {
    let input = parse(input);
    println!("{:?}", input);
}

fn main() -> Result<()> {
    let sample = read_to_string("sample")?;
    // let input = read_to_string("input")?;

    println!("{:?}", sample);
    part_one(&sample);
    // part_one(&input);
    Ok(())
}
