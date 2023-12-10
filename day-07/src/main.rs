use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

fn parse(input: &str) -> Result<Vec<(&str, usize)>, Box<dyn Error>> {
    let data = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            (hand, bid.parse::<usize>().unwrap())
        })
        .collect();

    Ok(data)
}

fn get_hand(cards: &str) -> usize {
    let mut map = HashMap::new();
    let set = cards.chars().collect::<HashSet<char>>();

    if set.len() == 1 {
        return 6;
    }
    if set.len() == 4 {
        return 1;
    }
    if set.len() == 5 {
        return 0;
    }

    for card in cards.chars() {
        *map.entry(card).or_insert(0) += 1;
    }

    let mut v = map.values().collect::<Vec<&usize>>();
    v.sort();

    if *v[v.len() - 1] == 4 {
        return 5;
    }
    if *v[v.len() - 1] == 3 && *v[0] == 2 {
        return 4;
    }
    if *v[v.len() - 1] == 3 && *v[0] == 1 {
        return 3;
    }

    2
}

fn sort_hands(ranked: &mut Vec<(&str, usize, usize)>) {
    let cards = "123456789TJQKA";
    ranked.sort_by_key(|&(_, r, _)| r);

    let mut indices_to_swap = Vec::new();

    for idx in 0..ranked.len().saturating_sub(1) {
        let (left, right) = (&ranked[idx], &ranked[idx + 1]);

        if left.1 == right.1 {
            let mut i = 0;

            while left.0.chars().nth(i) == right.0.chars().nth(i) {
                i += 1;
            }

            if i < left.0.len()
                && cards.find(left.0.chars().nth(i).unwrap())
                    > cards.find(right.0.chars().nth(i).unwrap())
            {
                indices_to_swap.push(idx);
            }
        }
    }

    for &idx in &indices_to_swap {
        ranked.swap(idx, idx + 1);
    }
}

fn part_one(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut winings = 0;
    let data = parse(input)?;

    let mut ranked: Vec<(&str, usize, usize)> = data
        .iter()
        .map(|(hand, bid)| (*hand, get_hand(hand), *bid))
        .collect();

    sort_hands(&mut ranked);

    for (factor, (_, _, bid)) in ranked.iter().enumerate() {
        winings += bid * (factor + 1);
    }

    Ok(winings)
}

fn main() -> Result<(), Box<dyn Error>> {
    let sample = read_to_string("sample")?;
    let input = read_to_string("input")?;

    println!("{:?}", part_one(&sample));
    println!("{:?}", part_one(&input));

    Ok(())
}
