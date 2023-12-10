use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

struct Almanac<'a> {
    keys: Vec<(&'a str, &'a str)>,
    seeds: Vec<u64>,
    _elements: HashSet<&'a str>,
    map: HashMap<(&'a str, &'a str), Vec<Vec<u64>>>,
}

fn parse(input: &String) -> Almanac {
    let mut seeds = Vec::<u64>::new();

    let mut _elements = HashSet::<&str>::new();
    let mut keys = Vec::<(&str, &str)>::new();
    let mut map = HashMap::<(&str, &str), Vec<Vec<u64>>>::new();

    for line in input.lines() {
        if line.contains("seeds: ") {
            if let Some((_, right)) = line.split_once(' ') {
                seeds = right
                    .split_whitespace()
                    .map(|e| e.parse::<u64>().unwrap_or(0))
                    .collect();
            }
        }

        if line.contains("-to-") {
            if let Some((pair, _)) = line.split_once(" ") {
                if let Some((left, right)) = pair.split_once("-to-") {
                    _elements.insert(right);
                    _elements.insert(left);
                    keys.push((left, right));
                    map.insert((left, right), Vec::<Vec<u64>>::new());
                }
            }
        } else if !line.is_empty() {
            if let Some((left, right)) = keys.last() {
                if let Some(k) = map.get_mut(&(left, right)) {
                    k.push(
                        line.split_whitespace()
                            .map(|d| d.parse().unwrap_or(0))
                            .collect(),
                    );
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

fn part_one(almanac: &Almanac) -> Result<u64, Box<dyn Error>> {
    let mut ans = u64::MAX;

    for seed in &almanac.seeds {
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

fn ranges_to_ranges(maps: &Vec<Vec<u64>>, ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut out = Vec::<(u64, u64)>::new();

    for range in ranges {
        let mut ret = Vec::<(u64, u64)>::new();
        for map in maps {
            let map_a = map[1];
            let map_b = map[1] + map[2];

            let range_a = range.0;
            let range_b = range.0 + range.1;
            let range_len = range.1;

            if (map_a..map_b).contains(&range_a) && (map_a..map_b).contains(&range_b) {
                let new_range_a = range_a - map[1] + map[0];
                ret.push((new_range_a, range_len));
            } else if (map_a..map_b).contains(&range_a) {
                let new_range_a = range_a - map[1] + map[0];
                ret.push((new_range_a, range_len - (range_b - map_b)));
                ret.extend(ranges_to_ranges(maps, &vec![(map_b + 1, range_b - map_b)].to_vec()));
            } else if (map_a..map_b).contains(&range_b) {
                let new_map_a = map_a - map[1] + map[0];
                ret.push((new_map_a, range_b - map_a));
                if range_a != 0 {
                    ret.extend(ranges_to_ranges(maps, &vec![(range_a - 1, map_a - range_a)].to_vec()));
                }
            }
        }
        if ret.is_empty() {
            ret.push(*range);
        }
        out.extend(ret);
    }
    out
}

fn part_two(almanac: &Almanac) -> u64 {
    let mut ans = Vec::<(u64, u64)>::new();

    for seed_pair in almanac.seeds.chunks(2) {
        let mut seed_vec = vec![(seed_pair[0], seed_pair[1])].to_vec();
        for key in &almanac.keys {
            if let Some(maps) = almanac.map.get(&key) {
                seed_vec = ranges_to_ranges(&maps, &seed_vec);
            }
        }
        let min_queue = seed_vec.iter().min_by_key(|&(left, _)| left).unwrap();
        ans.push(min_queue.clone());
    }

    let min = &ans.iter().filter(|&&(left, _)| left != 0).min_by_key(|&(left, _)| left);

    match min {
        Some(ret) => ret.0,
        None => 0,
    }
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
