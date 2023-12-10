use std::error::Error;
use std::fs::read_to_string;

fn parse_one(input: &str) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    let mut data = Vec::new();
    let mut out = Vec::new();

    for line in input.lines() {
        let parts = line.split_whitespace();
        let mut tmp = Vec::new();
        for part in parts {
            if let Ok(num) = part.parse::<usize>() {
                tmp.push(num);
            }
        }
        data.push(tmp);
    }

    for i in 0..data[0].len() {
        out.push((data[0][i], data[1][i]));
    }

    Ok(out)
}

fn part_one(input: &str) -> usize {
    let mut total = 1;
    let races: Vec<(usize, usize)> = parse_one(input).unwrap();

    for (time, distance) in races {
        let mut count = 0;
        for speed in 1..time {
            let travels = speed * (time - speed);
            if travels > distance {
                count += 1;
            }
        }
        total *= count;
    }

    total
}

fn parse_two(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let mut values = input.lines().map(|line| {
        let (_, data) = line.split_once(":").unwrap_or(("", ""));
        data.chars()
            .filter(|&c| !c.is_whitespace())
            .collect::<String>()
            .parse::<usize>()
    });

    let time = values.next().ok_or("Missing time value")??;
    let distance = values.next().ok_or("Missing distance value")??;

    Ok((time, distance))
}

fn part_two(input: &String) -> usize {
    let (time, distance) = parse_two(input).unwrap();

    let mut count = 0;
    for speed in 1..time {
        let travels = speed * (time - speed);
        if travels > distance {
            count += 1;
        }
    }

    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let sample = read_to_string("sample")?;
    let input = read_to_string("input")?;

    println!("Part One Sample: {:?}", part_one(&sample));
    println!("Part One Input: {:?}", part_one(&input));

    println!("Part Two Sample: {:?}", part_two(&sample));
    println!("Part Two Input: {:?}", part_two(&input));

    Ok(())
}
