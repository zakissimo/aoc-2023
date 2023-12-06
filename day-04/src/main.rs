use std::error::Error;
use std::fs::read_to_string;

fn parse(input: &String) -> Result<Vec<(u32, Vec<u32>, Vec<u32>)>, Box<dyn Error>> {
    let mut vec = Vec::new();

    for line in input.lines() {
        if let Some((card, data)) = line.split_once(": ") {
            if let Some((_, num)) = card.split_once(' ') {

                let num = num.trim().parse::<u32>().unwrap();

                if let Some((win, mine)) = data.split_once(" | ") {
                    let win: Vec<u32> = win
                        .split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect();
                    let mine: Vec<u32> = mine
                        .split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect();

                    vec.push((num, win, mine));
                }
            }
        }
    }

    Ok(vec)
}

fn part_one(input: &String) -> Result<u32, Box<dyn Error>> {
    let mut ans = 0;
    let vec = parse(input)?;

    for (_, win, mine) in vec {
        let mine: Vec<&u32> = mine.iter().filter(|&x| win.contains(x)).collect();
        if mine.len() > 0 {
            ans += 2_u32.pow((mine.len() - 1) as u32);
        }
    }

    Ok(ans)
}

fn part_two(input: &String) -> Result<u32, Box<dyn Error>> {
    let vec = parse(input)?;
    let mut freq = vec![1; vec.len()];

    for (num, win, mine) in vec {
        let mine: Vec<&u32> = mine.iter().filter(|&x| win.contains(x)).collect();
        let start = num + 1;
        let end = num + mine.len() as u32;
        if mine.len() > 0 {
            for i in start..=end {
                freq[i as usize - 1] += freq[num as usize - 1];
            }
        }
    }

    Ok(freq.iter().fold(0, |acc, &x| acc + x))
}

fn main() -> Result<(), Box<dyn Error>> {

    let sample = read_to_string("sample")?;
    let input = read_to_string("input")?;
    //
    println!("Part one sample {:?}", part_one(&sample)?);
    println!("Part one input {:?}", part_one(&input)?);

    println!("Part two sample {:?}", part_two(&sample)?);
    println!("Part two input {:?}", part_two(&input)?);
    Ok(())

}
