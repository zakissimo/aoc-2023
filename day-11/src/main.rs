use anyhow::Result;
use std::fs::read_to_string;

fn parse(input: &str) -> Vec<Vec<char>> {
    let universe: Vec<Vec<char>> = input.lines().map(|row| {
        row.chars().collect()
    }).collect();

    universe
}

fn expand(universe: &mut Vec<Vec<char>>) {

    let mut insert_at = Vec::<usize>::new();
    for (i, row) in universe.iter_mut().enumerate() {
        if row.iter().all(|&c| c == '.') {
            insert_at.push(i);
        }
    }
    for i in insert_at {
        universe.insert(i, universe[i].clone());
    }

    let mut insert_at = Vec::<usize>::new();
    for x in 0..universe[0].len() {
        for y in 0..universe.len() {
            if universe[y][x] != '.' {
                break;
            }
            if y == universe.len() - 1 {
                insert_at.push(x);
            }
        }
    }

    for i in insert_at {
        for row in universe.iter_mut() {
            row.insert(i, '.');
        }
    }
}

fn part_one(input: &str) {
    let mut universe = parse(input);
    expand(&mut universe);
}

fn main() -> Result<()> {
    let sample = read_to_string("sample")?;
    let input = read_to_string("input")?;

    part_one(&sample);

    Ok(())
}
