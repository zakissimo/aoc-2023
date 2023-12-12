use std::error::Error;
use std::fs::read_to_string;

fn parse(input: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let lines = input.lines()
            .map(|line| line
                        .split_whitespace()
                        .map(|n| n.parse::<i32>()
                            .expect("Input should be i32"))
                            .collect()
            ).collect();

    Ok(lines)
}

fn generate_sequences(meta: &mut Vec<Vec<i32>>) {

    let mut new = Vec::<i32>::new();
    if let Some(last) = meta.last() {
        for window in last.windows(2) {
            if let [left, right] = window {
                new.push(right - left);
            }
        }
    }
    let sum = new.iter().sum::<i32>();
    if sum != 0 {
        meta.push(new);
        generate_sequences(meta);
    }
}

fn get_next(meta: Vec<Vec<i32>>) -> i32 {
    let mut acc = 0;

    for v in meta.iter().rev() {
        if let Some(last) = v.last() {
            acc += last;
        }
    }

    acc
}

fn part_one(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut ans = 0;
    let data: Vec<Vec<i32>> = parse(input)?;

    for seq in data {
        let mut meta = Vec::<Vec<i32>>::new();
        meta.push(seq);
        generate_sequences(&mut meta);
        ans += get_next(meta);
    }
    Ok(ans)
}

fn main() -> Result<(), Box<dyn Error>> {

    let sample = read_to_string("sample")?;
    let input = read_to_string("input")?;

    println!("Part One Sample: {:?}", part_one(&sample));
    println!("Part One input: {:?}", part_one(&input));

    Ok(())
}
