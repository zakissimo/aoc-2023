use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

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

fn get_next(
    dirs: &[(i32, i32); 2],
    grid: &Vec<&[u8]>,
    prev: &mut (usize, usize),
    curr: &mut (usize, usize),
) -> Option<(usize, usize)> {
    let diff: (i32, i32) = (curr.0 as i32 - prev.0 as i32, curr.1 as i32 - prev.1 as i32);
    for dir in dirs {
        if *dir != diff
            && is_inbound(
                grid,
                (curr.0 as i32 + dir.0) as usize,
                (curr.1 as i32 + dir.1) as usize,
            )
        {
            *prev = *curr;
            let next = (
                (curr.0 as i32 + dir.0) as usize,
                (curr.1 as i32 + dir.1) as usize,
            );
            return Some(next);
        }
    }
    None
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

fn walk(grid: &Vec<&[u8]>, start: (usize, usize)) -> usize {
    let pipe_list = vec![b'|', b'-', b'L', b'J', b'7', b'F'];
    let pipes: HashMap<u8, [(i32, i32); 2]> = HashMap::from([
        (b'|', [(1, 0), (-1, 0)]),
        (b'-', [(0, -1), (0, 1)]),
        (b'L', [(1, 0), (0, 1)]),
        (b'J', [(1, 0), (0, -1)]),
        (b'7', [(-1, 0), (0, -1)]),
        (b'F', [(-1, 0), (0, 1)]),
    ]);
    let dirs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut ways = Vec::<(usize, usize)>::new();
    for dir in dirs {
        if is_inbound(
            grid,
            (start.0 as i32 + dir.0) as usize,
            (start.1 as i32 + dir.1) as usize,
        ) {
            ways.push((
                (start.0 as i32 + dir.0) as usize,
                (start.1 as i32 + dir.1) as usize,
            ));
        }
    }
    println!("> {:?}", ways);
    let mut ret = 0;
    println!("{:?}", grid);
    for way in ways {
        let mut prev = start;
        let mut curr = way;
        let mut ans = 0;
        if pipe_list.contains(&grid[curr.0][curr.1]) {
            while let Some(next) = get_next(
                pipes.get(&grid[curr.0][curr.1]).unwrap(),
                grid,
                &mut prev,
                &mut curr,
            ) {
                println!("{}", grid[next.0][next.1]);
                if grid[next.0][next.1] == b'S' || ans == 5 {
                    break;
                }
                ans += 1;
            }
        }
        ret = ans;
    }
    ret
}

fn part_one(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut ans = 0;
    let grid = parse(input)?;

    println!("{:?}", grid);
    if let Some(start) = get_start(&grid) {
        println!("{:?}", start);
        ans = walk(&grid, start);
    }

    Ok(ans)
}

fn main() -> Result<(), Box<dyn Error>> {
    let sample_one = read_to_string("sample_one")?;
    // let sample_two = read_to_string("sample_two")?;
    // let input = read_to_string("input")?;
    println!("Sample one: {}", part_one(&sample_one)?);

    Ok(())
}
