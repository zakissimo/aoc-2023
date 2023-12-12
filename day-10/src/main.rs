use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone)]
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

    fn from_tuple(tuple: (i32, i32)) -> Option<Dir> {
        match tuple {
            (0, -1) => Some(Dir::UP),
            (0, 1) => Some(Dir::DOWN),
            (-1, 0) => Some(Dir::LEFT),
            (1, 0) => Some(Dir::RIGHT),
            _ => None,
        }
    }
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

fn walk(grid: &Vec<&[u8]>, prev: &mut (usize, usize), curr: &mut (usize, usize)) {
    let mut visited: Vec<Vec<bool>> = grid.iter().map(|row| vec![false; row.len()]).collect();
    let pipes: HashMap<u8, [Dir; 2]> = HashMap::from([
        (b'|', [Dir::UP, Dir::DOWN]),
        (b'-', [Dir::LEFT, Dir::RIGHT]),
        (b'L', [Dir::UP, Dir::RIGHT]),
        (b'J', [Dir::UP, Dir::LEFT]),
        (b'7', [Dir::DOWN, Dir::LEFT]),
        (b'F', [Dir::DOWN, Dir::RIGHT]),
    ]);

    let diff: (i32, i32) = (curr.0 as i32 - prev.0 as i32, curr.1 as i32 - prev.1 as i32);
    if let Some(dir) = Dir::from_tuple(diff) {}
}

fn part_one(input: &str) -> Result<usize, Box<dyn Error>> {
    // let mut ans = 0;
    let grid = parse(input)?;

    println!("{:?}", grid);
    if let Some(start) = get_start(&grid) {
        let mut ways = Vec::<(usize, usize)>::new();
        for dir in Dir::LIST {
            let x = (start.0 as i32 + dir.dx) as usize;
            let y = (start.1 as i32 + dir.dy) as usize;
            if is_inbound(&grid, x, y) && is_pipe(grid[y][x]) {
                ways.push((x, y));
            }
        }

        for (x, y) in &ways {}

        println!("{:?}", ways);
        println!("{:?}", start);
    }

    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let sample_one = read_to_string("sample_one")?;
    // let sample_two = read_to_string("sample_two")?;
    // let input = read_to_string("input")?;
    println!("Sample one: {}", part_one(&sample_one)?);

    Ok(())
}
