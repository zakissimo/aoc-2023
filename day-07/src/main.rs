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

fn get_hand_one(cards: &str) -> usize {
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

fn get_hand_two(cards: &str, rank: usize) -> usize {
    let mut map = HashMap::new();

    for card in cards.chars() {
        *map.entry(card).or_insert(0) += 1;
    }

    let mut v = map.values().collect::<Vec<&usize>>();
    v.sort();

    let set = cards.chars().collect::<HashSet<char>>();

    if let Some(j_count) = map.get(&'J') {
        // JJJJA
        if *j_count >= 4 {
            return 6;
        }

        // JJJKA / JJJAA
        if *j_count == 3 {
            if set.len() == 2 {
                return 6;
            }
            if set.len() == 3 {
                return 5;
            }
        }

        // AKQJJ / KAAJJ / AAAJJ
        if *j_count == 2 {
            if set.len() == 4 {
                return 3;
            }
            if set.len() == 3 {
                return 5;
            }
            if set.len() == 2 {
                return 6;
            }
        }

        // AAQTJ / AKQTJ / AAKKJ / AAAKJ / AAAAJ
        if *j_count == 1 {
            if rank == 1 {
                return 3;
            }
            if rank == 2 {
                return 4;
            }
            if rank == 3 {
                return 5;
            }
            if set.len() == 2 {
                return 6;
            }
            if set.len() == 5 {
                return 1;
            }
        }
    }

    rank
}

fn sort_hands(cards: &str, ranked: &mut Vec<(&str, usize, usize)>) {
    ranked.sort_by(|(hand1, rank1, _), (hand2, rank2, _)| {
        let cmp_by_rank = rank1.cmp(rank2);

        if cmp_by_rank == std::cmp::Ordering::Equal {
            for (a, b) in hand1.chars().zip(hand2.chars()) {
                if a != b {
                    return cards.find(a).unwrap().cmp(&cards.find(b).unwrap());
                }
            }
        }
        cmp_by_rank
    });
}

fn part_one(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut winnings = 0;
    let data = parse(input)?;
    let cards = "23456789TJQKA";

    let mut ranked: Vec<(&str, usize, usize)> = data
        .iter()
        .map(|(hand, bid)| (*hand, get_hand_one(hand), *bid))
        .collect();

    sort_hands(cards, &mut ranked);

    for (factor, (_, _, bid)) in ranked.iter().enumerate() {
        winnings += bid * (factor + 1);
    }

    Ok(winnings)
}

fn part_two(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut winnings = 0;
    let data = parse(input)?;
    let cards = "J23456789TQKA";

    let mut ranked: Vec<(&str, usize, usize)> = data
        .iter()
        .map(|(hand, bid)| (*hand, get_hand_two(hand, get_hand_one(hand)), *bid))
        .collect();

    sort_hands(cards, &mut ranked);

    for (factor, (_, _, bid)) in ranked.iter().enumerate() {
        winnings += bid * (factor + 1);
    }

    Ok(winnings)
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
