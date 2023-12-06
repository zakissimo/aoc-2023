use std::fs;
use std::error::Error;
use std::collections::HashSet;

fn get_boundaries(line: &str, idx: usize) -> (usize, usize) {
    let mut start = idx;
    let mut end = idx;

    while start > 0 && line.chars().nth(start).unwrap_or('.').is_digit(10) {
        start -= 1;
    }
    if !line.chars().nth(start).unwrap_or('.').is_digit(10) {
        start += 1;
    }
    while end < line.len() && line.chars().nth(end).unwrap_or('.').is_digit(10) {
        end += 1;
    }

    (start, end)
}

fn look_in_direction(lines: &Vec<&str>, symbol: &(usize, usize), dy: isize, dx: isize) -> Option<(usize, usize, usize)> {
    let y = (symbol.0 as isize + dy) as usize;
    let x = (symbol.1 as isize + dx) as usize;

    if y < lines.len() && x < lines[y].len() {
        if lines[y].chars().nth(x).unwrap_or('.').is_digit(10) {
            let boundaries = get_boundaries(lines[y], x);
            Some((y, boundaries.0, boundaries.1))
        } else {
            None
        }
    } else {
        None
    }
}

fn find_parts_one(lines: &Vec<&str>, symbol_loc: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut parts: Vec<usize> = Vec::new();
    let mut set: HashSet<(usize, usize, usize)> = HashSet::new();

    for symbol in symbol_loc {
        let directions = vec![(0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1)];

        for (dy, dx) in directions.iter() {
            if let Some(part) = look_in_direction(lines, symbol, *dy, *dx) {
                set.insert(part);
            }
        }
    }

    for raw_part in set {
        parts.push(lines[raw_part.0][raw_part.1..raw_part.2].parse::<usize>().unwrap());
    }

    parts
}

fn get_symbol_loc<F>(lines: &Vec<&str>, is_symbol: F) -> Vec<(usize, usize)>
where
    F: Fn(&char) -> bool,
{
    let mut symbol_loc: Vec<(usize, usize)> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if is_symbol(&c) {
                symbol_loc.push((i, j));
            }
        }
    }
    symbol_loc
}

fn find_parts_two(lines: &Vec<&str>, symbol_loc: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut parts: Vec<usize> = Vec::new();

    for symbol in symbol_loc {
        let mut set: HashSet<(usize, usize, usize)> = HashSet::new();
        let directions = vec![(0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1)];

        for (dy, dx) in directions.iter() {
            if let Some(part) = look_in_direction(lines, symbol, *dy, *dx) {
                set.insert(part);
            }
        }
        if set.len() == 2 {
            parts.push(set.iter().fold(1, |acc, x| acc * lines[x.0][x.1..x.2].parse::<usize>().unwrap()))
        }
    }
    parts
}

fn part_two(input: &String) -> Result<usize, Box<dyn Error>> {

    let lines: Vec<&str> = input.split("\n").collect();

    let is_symbol = |&c: &char| c == '*';

    let symbol_loc = get_symbol_loc(&lines, is_symbol);
    let parts = find_parts_two(&lines, &symbol_loc);

    Ok(parts.iter().fold(0, |acc, x| acc + x))

}

fn part_one(input: &String) -> Result<usize, Box<dyn Error>> {

    let lines: Vec<&str> = input.split("\n").collect();

    let is_symbol = |&c: &char| c != '.' && !c.is_digit(10);

    let symbol_loc = get_symbol_loc(&lines, is_symbol);
    let parts = find_parts_one(&lines, &symbol_loc);

    Ok(parts.iter().fold(0, |acc, x| acc + x))

}

fn main() -> Result<(), Box<dyn Error>> {
    let sample = fs::read_to_string("sample")?;
    let input = fs::read_to_string("input")?;

    println!("Part one sample {}", part_one(&sample)?);
    println!("Part one input {}", part_one(&input)?);

    println!("Part two sample {}", part_two(&sample)?);
    println!("Part two input {}", part_two(&input)?);
    Ok(())
}
