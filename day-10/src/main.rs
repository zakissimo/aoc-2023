use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Dir {
    dx: i32,
    dy: i32,
}

impl Dir {
    const UP: Dir = Dir { dy: -1, dx: 0 };
    const DOWN: Dir = Dir { dy: 1, dx: 0 };
    const LEFT: Dir = Dir { dy: 0, dx: -1 };
    const RIGHT: Dir = Dir { dy: 0, dx: 1 };
    const LIST: [Dir; 4] = [Dir::UP, Dir::DOWN, Dir::LEFT, Dir::RIGHT];
}

fn parse(input: &str) -> Result<Vec<&[u8]>, Box<dyn Error>> {
    Ok(input.lines().map(|line| line.as_bytes()).collect())
}

fn is_inbound(grid: &Vec<&[u8]>, x: usize, y: usize) -> bool {
    if let Some(row) = grid.get(y) {
        if let Some(_) = row.get(x) {
            return true;
        }
    }
    false
}

fn is_pipe(c: u8) -> bool {
    return "|-LJ7F".contains(c as char);
}

fn get_start(grid: &Vec<&[u8]>) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == b'S' {
                return Some((j, i));
            }
        }
    }
    println!("No start found");
    None
}

fn rev_dir(dir: Dir) -> Dir {
    match dir {
        Dir::UP => Dir::DOWN,
        Dir::DOWN => Dir::UP,
        Dir::LEFT => Dir::RIGHT,
        Dir::RIGHT => Dir::LEFT,
        _ => panic!("Unknown dir"),
    }
}

fn walk(
    grid: &Vec<&[u8]>,
    visited: &mut Vec<Vec<bool>>,
    prev: &mut (usize, usize),
    go: &mut Dir,
) -> Option<bool> {
    let pipes: HashMap<u8, [Dir; 2]> = HashMap::from([
        (b'|', [Dir::UP, Dir::DOWN]),
        (b'-', [Dir::LEFT, Dir::RIGHT]),
        (b'L', [Dir::UP, Dir::RIGHT]),
        (b'J', [Dir::UP, Dir::LEFT]),
        (b'7', [Dir::DOWN, Dir::LEFT]),
        (b'F', [Dir::DOWN, Dir::RIGHT]),
    ]);

    let curr = (
        (prev.0 as i32 + go.dx) as usize,
        (prev.1 as i32 + go.dy) as usize,
    );
    if is_inbound(grid, curr.0, curr.1) {
        visited[curr.1][curr.0] = true;
        if let Some(pipe) = pipes.get(&grid[curr.1][curr.0]) {
            let rev_pipe = pipe.iter().map(|d| rev_dir(*d)).collect::<Vec<Dir>>();
            if rev_pipe.contains(&go) {
                if let Some(next_dir) = pipe.iter().find(|dir| **dir != rev_dir(*go)) {
                    let next = (
                        (curr.0 as i32 + next_dir.dx) as usize,
                        (curr.1 as i32 + next_dir.dy) as usize,
                    );
                    if is_inbound(&grid, next.0, next.1) {
                        if !visited[next.1 as usize][next.0 as usize]
                            && is_pipe(grid[next.1][next.0])
                        {
                            *go = *next_dir;
                            *prev = curr;
                            return Some(true);
                        }
                        if grid[next.1][next.0] == b'S' {
                            return Some(false);
                        }
                    }
                }
            }
        }
    }
    None
}

fn part_one(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut ans = 0;
    let grid = parse(input)?;

    if let Some(start) = get_start(&grid) {
        for dir in &Dir::LIST {
            let mut prev = start;
            let mut go = dir.clone();
            let mut visited: Vec<Vec<bool>> =
                grid.iter().map(|row| vec![false; row.len()]).collect();
            visited[start.1][start.0] = true;
            let mut count = 1;
            loop {
                if let Some(walk) = walk(&grid, &mut visited, &mut prev, &mut go) {
                    count += 1;
                    if !walk {
                        ans = count;
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    Ok(ans / 2)
}

fn get_inside(grid: &Vec<&[u8]>, visited: &mut Vec<Vec<bool>>, start_pipe: u8) -> usize {
    let mut ans = 0;
    let mut tiles = 0;
    let pipes = [b'F', b'7', b'L', b'J'];
    let down_pipes = [b'F', b'7'];
    let up_pipes = [b'L', b'J'];
    for (y, row) in grid.iter().enumerate() {
        let mut inside = false;
        let mut prev = b'*';
        ans += tiles;
        tiles = 0;
        let mut seen = 0;
        for (x, tile) in row.iter().enumerate() {
            if x < row.len() - 1 {
                let mut tile = tile;
                if *tile == b'S' {
                    tile = &start_pipe;
                    println!("S pipe is {}", *tile as char);
                }
                if visited[y][x] {
                    if *tile == b'|' {
                        inside = !inside;
                    } else if pipes.contains(tile) {
                        seen += 1;
                        if pipes.contains(&prev) {
                            if seen % 2 == 0
                                && (down_pipes.contains(&prev) && up_pipes.contains(&tile))
                                || (down_pipes.contains(&tile) && up_pipes.contains(&prev))
                            {
                                inside = !inside;
                            }
                        }
                        prev = *tile;
                    }
                } else {
                    if inside {
                        tiles += 1;
                        // println!("{} {} {}", y, x, *tile as char);
                    }
                }
                println!(
                    "Inside: {}, y: {}, x: {}, tile: {}",
                    inside, y, x, *tile as char
                );
            }
        }
    }
    ans
}

fn dir_to_pipe(target_value: &Vec<Dir>) -> Option<u8> {
    let pipes: HashMap<u8, [Dir; 2]> = HashMap::from([
        (b'|', [Dir::UP, Dir::DOWN]),
        (b'-', [Dir::LEFT, Dir::RIGHT]),
        (b'L', [Dir::UP, Dir::RIGHT]),
        (b'J', [Dir::UP, Dir::LEFT]),
        (b'7', [Dir::DOWN, Dir::LEFT]),
        (b'F', [Dir::DOWN, Dir::RIGHT]),
    ]);
    for (key, value) in pipes.iter() {
        if value.contains(&target_value[0]) && value.contains(&target_value[1]) {
            return Some(*key);
        };
    }
    None
}

fn part_two(input: &str) -> Result<usize, Box<dyn Error>> {
    let grid = parse(input)?;

    let mut start_pipe = Vec::<Dir>::new();
    let mut visited: Vec<Vec<bool>> = grid.iter().map(|row| vec![false; row.len()]).collect();

    if let Some(start) = get_start(&grid) {
        for dir in &Dir::LIST {
            let mut prev = start;
            let mut go = dir.clone();
            visited = grid.iter().map(|row| vec![false; row.len()]).collect();
            visited[start.1][start.0] = true;
            loop {
                if let Some(walk) = walk(&grid, &mut visited, &mut prev, &mut go) {
                    if !walk {
                        start_pipe.push(*dir);
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    Ok(get_inside(
        &grid,
        &mut visited,
        dir_to_pipe(&start_pipe).unwrap(),
    ))
}

fn main() -> Result<(), Box<dyn Error>> {
    // let sample_one = read_to_string("sample_one")?;
    // let sample_two = read_to_string("sample_two")?;
    let sample_three = read_to_string("sample_three")?;
    let sample_four = read_to_string("sample_four")?;
    let sample_five = read_to_string("sample_five")?;
    let input = read_to_string("input")?;

    // println!("Sample one: {}", part_one(&sample_one)?);
    // println!("Sample two: {}", part_one(&sample_two)?);

    println!("Sample three: {}", part_two(&sample_three)?);
    println!("Sample four: {}", part_two(&sample_four)?);
    println!("Sample five: {}", part_two(&sample_five)?);

    // println!("Input: {}", part_one(&input)?);
    // println!("Input: {}", part_two(&input)?);

    Ok(())
}
