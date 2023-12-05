use std::fs;
use std::error::Error;
use std::collections::HashMap;

fn part_two(input: &String) -> Result<usize, Box<dyn Error>> {

    let mut ans = 0;

    for line in input.lines() {

        let (_, game) = line.split_once(":").unwrap();

        let mut cube_freq = HashMap::from([
            ("red", 1),
            ("green", 1),
            ("blue", 1),
        ]);

        for round in game.split(';') {

            for cube_data in round.split(',') {
                let (freq, color) = cube_data.trim().split_once(' ').unwrap();
                let freq = freq.parse::<usize>().unwrap();
                cube_freq.insert(color, cube_freq
                    .get(color)
                    .map_or(freq, |&existing_freq| existing_freq
                        .max(freq)));
            }
        }

        ans += cube_freq.values().fold(1, |acc, &val| acc * val);

    }

    Ok(ans)
}

fn part_one(input: &String) -> Result<usize, Box<dyn Error>> {

    let mut ans = 0;

    let look_up = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    for line in input.lines() {

        let (id, game) = line.split_once(":").unwrap();

        let id = id.chars().skip(5).collect::<String>().parse::<usize>().unwrap();

        let mut valid = true;
        for round in game.split(';') {

            let mut cube_freq = HashMap::from([
                ("red", 0),
                ("green", 0),
                ("blue", 0),
            ]);
            for cube_data in round.split(',') {
                let (freq, color) = cube_data.trim().split_once(' ').unwrap();
                cube_freq.entry(color)
                    .and_modify(|v| *v += freq.parse::<usize>().unwrap());
                if *cube_freq.get(color).unwrap() > *look_up.get(color).unwrap() {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            ans += id;
        }

    }

    Ok(ans)
}

fn main() -> Result<(), Box<dyn Error>> {

    let sample = fs::read_to_string("sample")?;
    let input = fs::read_to_string("input")?;

    println!("Part one sample: {}", part_one(&sample)?);
    println!("Part one input: {}", part_one(&input)?);
    println!("Part two sample: {}", part_two(&sample)?);
    println!("Part two input: {}", part_two(&input)?);

    Ok(())
}
