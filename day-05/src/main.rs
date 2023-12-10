use std::error::Error;
use std::fs::read_to_string;
use std::collections::HashSet;
use std::collections::HashMap;

struct Almanac<'a> {
    keys: Vec<(&'a str, &'a str)>,
    seeds: Vec<usize>,
    _elements: HashSet<&'a str>,
    map: HashMap<(&'a str, &'a str), Vec<Vec<usize>>>,
}

fn parse(input: &String) -> Almanac {
    let mut seeds = Vec::<usize>::new();

    let mut _elements = HashSet::<&str>::new();
    let mut keys = Vec::<(&str, &str)>::new();
    let mut map = HashMap::<(&str, &str), Vec<Vec<usize>>>::new();

    for line in input.lines() {

        if line.contains("seeds: ") {
                if let Some((_, right)) = line.split_once(' '){
                    seeds = right.split_whitespace().map(|e| e.parse::<usize>().unwrap_or(0)).collect();
            }
        }

        if line.contains("-to-") {
            if let Some((pair, _)) = line.split_once(" ") {
                if let Some((left, right)) = pair.split_once("-to-") {
                    _elements.insert(right);
                    _elements.insert(left);
                    keys.push((left, right));
                    map.insert((left, right), Vec::<Vec<usize>>::new());
                }
            }
        } else if !line.is_empty() {
            if let Some((left, right)) = keys.last() {
                if let Some(k) = map.get_mut(&(left, right)) {
                    k.push(line.split_whitespace().map(|d| d.parse().unwrap_or(0)).collect());
                }
            }
        }
    }

    Almanac {
        keys,
        seeds,
        _elements,
        map,
    }
}

fn part_one(almanac: &Almanac) -> Result<usize, Box<dyn Error>> {

    let mut ans = usize::MAX;
    let mut count = 0;
    let total = almanac.seeds.len();

    for seed in &almanac.seeds {
        count += 1;
        println!("> {:?}", count as f64 / total as f64 * 100.0);
        let mut queue = vec![*seed];
        for key in &almanac.keys {
            if let Some(v) = almanac.map.get(&key) {
                let curr = queue.pop().unwrap();
                for vec in v {
                    if (vec[1]..(vec[1] + vec[2])).contains(&curr) {
                        queue.push(vec[0] + curr - vec[1]);
                    }
                }
                if queue.is_empty() {
                    queue.push(curr);
                }
            }
        }
        ans = ans.min(queue.pop().unwrap());
    }
    Ok(ans)
}

fn part_two(almanac: &Almanac) -> Result<usize, Box<dyn Error>> {

    let mut seeds = HashSet::<usize>::new();

    for seed_pair in almanac.seeds.chunks(2) {
        for seed in seed_pair[0]..(seed_pair[0] + seed_pair[1]) {
            seeds.insert(seed);
        }
    }

    let seeds: Vec::<usize> = seeds.iter().cloned().collect();

    let new_almanac = Almanac {
        keys: almanac.keys.clone(),
        seeds,
        _elements: almanac._elements.clone(),
        map: almanac.map.clone(),
    };

    Ok(part_one(&new_almanac)?)
}

fn main() -> Result<(), Box<dyn Error>> {

    let sample = read_to_string("sample")?;
    let input = read_to_string("input")?;

    let almanac_sample = parse(&sample);
    let almanac_input = parse(&input);

    println!("Part one sample: {:?}", part_one(&almanac_sample));
    println!("Part one input: {:?}", part_one(&almanac_input));

    println!("Part two sample: {:?}", part_two(&almanac_sample));
    println!("Part two input: {:?}", part_two(&almanac_input));

    Ok(())
}
