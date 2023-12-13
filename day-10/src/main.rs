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
                return Some((i, j));
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

    visited[prev.1][prev.0] = true;
    let curr = (
        (prev.0 as i32 + go.dx) as usize,
        (prev.1 as i32 + go.dy) as usize,
    );
    if is_inbound(grid, curr.0, curr.1) {
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

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn part_one(input: &str) -> Result<usize, Box<dyn Error>> {
    // let mut ans: (usize, f64) = (0, 0.0);
    let mut ans = 0;
    let grid = parse(input)?;

    if let Some(start) = get_start(&grid) {
        for dir in &Dir::LIST {
            let mut prev = start;
            let mut go = dir.clone();
            let mut points: Vec<Vec<(usize, f64)>> =
                grid.iter().map(|row| vec![(0, 0.0); row.len()]).collect();
            let mut visited: Vec<Vec<bool>> =
                grid.iter().map(|row| vec![false; row.len()]).collect();
            visited[start.1][start.0] = true;
            let mut count = 0;
            loop {
                if let Some(walk) = walk(&grid, &mut visited, &mut prev, &mut go) {
                    if !walk {
                        // ans = *points
                        //     .iter()
                        //     .map(|row| {
                        //         row.iter()
                        //             .max_by(|&a, &b| a.1.partial_cmp(&b.1).unwrap())
                        //             .unwrap()
                        //     })
                        //     .max_by(|&a, &b| a.1.partial_cmp(&b.1).unwrap())
                        //     .unwrap();
                        println!("{:?}", count);
                        println!("{:?}", visited);
                        ans = count;
                        break;
                    } else {
                        count += 1;
                        // points[prev.1][prev.0] = (
                        //     count,
                        //     distance(start.0 as f64, start.1 as f64, prev.0 as f64, prev.1 as f64),
                        // );
                    }
                } else {
                    break;
                }
            }
        }
    }

    Ok(ans)
}

fn main() -> Result<(), Box<dyn Error>> {
    let sample_one = read_to_string("sample_one")?;
    let sample_two = read_to_string("sample_two")?;
    let input = read_to_string("input")?;

    println!("Sample one: {}", part_one(&sample_one)?);
    println!("Sample two: {}", part_one(&sample_two)?);
    // println!("Input: {}", part_one(&input)?);

    Ok(())
}
