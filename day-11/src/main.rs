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
    for (idx, i) in insert_at.iter().enumerate() {
        universe.insert(i + idx, universe[i + idx].clone());
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

    for (idx, i) in insert_at.iter().enumerate() {
        for row in universe.iter_mut() {
            row.insert(*i + idx, '.');
        }
    }
}

fn locate_galaxies(universe: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::<(usize, usize)>::new();
    for (y, row) in universe.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxies.push((x, y));
            }
        }
    }
    galaxies
}

fn count_steps(galaxies: &Vec<(usize, usize)>) -> usize {
    let mut count = 0;
    for i in 0..(galaxies.len() - 1){
        for j in (i + 1)..galaxies.len() {
            count += ((galaxies[j].0 as i32 - galaxies[i].0 as i32).abs() as usize)
                    + ((galaxies[j].1 as i32 - galaxies[i].1 as i32).abs() as usize);
        }
    }

    count
}

fn part_one(input: &str) -> usize {
    let mut universe = parse(input);
    expand(&mut universe);
    let galaxies = locate_galaxies(&universe);

    count_steps(&galaxies)
}

fn find_empty_rows(universe: &Vec<Vec<char>>, y: usize) -> usize {
    let mut count = 0;

    for i in 0..y {
        if !universe[i].contains(&'#') {
            count += 1;
        }
    }

    count
}

fn find_empty_cols(universe: &Vec<Vec<char>>, x: usize) -> usize {
    let mut count = 0;

    for i in 0..x {
        for (j, row) in universe.iter().enumerate() {
            if row[i] == '#' {
                break;
            }
            if j == universe.len() - 1 {
                count += 1;
            }
        }
    }

    count
}

fn part_two(input: &str, expansion_factor: usize) -> usize {
    let universe = parse(input);
    let mut galaxies = locate_galaxies(&universe);

    for galaxy in galaxies.iter_mut() {
        galaxy.0 += expansion_factor * find_empty_cols(&universe, galaxy.0);
        galaxy.1 += expansion_factor * find_empty_rows(&universe, galaxy.1);
    }

    count_steps(&galaxies)
}

fn main() -> Result<()> {
    let sample = read_to_string("sample")?;
    let input = read_to_string("input")?;

    println!("Part One Sample: {:?}", part_one(&sample));
    println!("Part One Input: {:?}", part_one(&input));

    println!("Part Two Sample (10): {:?}", part_two(&sample, 10 - 1));
    println!("Part Two Sample (100): {:?}", part_two(&sample, 100 - 1));
    println!("Part Two Input: {:?}", part_two(&input, 1E6 as usize - 1));

    Ok(())
}
