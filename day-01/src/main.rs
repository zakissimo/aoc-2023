use std::fs;
use std::error::Error;
// use std::env::current_dir;

fn part_one(input: &str)  -> Result<usize, Box<dyn Error>> {
    let lines: Vec<&str> = input.split('\n').collect();

    let mut ans: usize = 0;

    for line in lines {
        if line.len() > 0 {
            let mut n = String::new();
            for c in line.chars() {
               if c.is_digit(10) {
                    n.push(c);
                    break;
               }
            }
            for c in line.chars().rev() {
               if c.is_digit(10) {
                    n.push(c);
                    break;
               }
            }
            ans += n.parse::<usize>()?;
        }
    }
    Ok(ans)
}

fn find_digits(line: &str) -> Vec<(usize, usize)> {
    let look_up = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut ret = Vec::new();

    for (digit, literal) in look_up.iter().enumerate() {
        if let Some(idx) = line.find(literal) {
            ret.push((idx, digit));
        }
    }

    for (digit, literal) in look_up.iter().enumerate() {
        if let Some(idx) = line.rfind(literal) {
            ret.push((idx, digit));
        }
    }

    for (idx, c) in line.chars().enumerate() {
       if c.is_digit(10) {
            ret.push((idx, c.to_digit(10).unwrap().try_into().unwrap()));
       }
    }

    ret.sort_by(|a, b| a.0.cmp(&b.0));

    ret
}

fn part_two(input: &str)  -> Result<usize, Box<dyn Error>> {
    let mut ans = 0;
    let lines: Vec<&str> = input.split('\n').collect();
    for line in lines {
        if line.len() > 0 {
            let digits = find_digits(line);
            // println!("{:?}", digits);
            let first = digits.first().unwrap().1;
            let last = digits.last().unwrap().1;
            // println!("{}, {}", first, last);
            ans += first * 10 + last;
        }
    }
    Ok(ans)
}

fn main() -> Result<(), Box<dyn Error>> {
    // println!("{:?}", current_dir());
    // let sample_one = fs::read_to_string("sample-one")?;
    let sample_two = fs::read_to_string("sample-two")?;
    let input = fs::read_to_string("input")?;

    // println!("{:?}", part_one(&sample_one));
    // println!("{:?}", part_one(&input));

    println!("{:?}", part_two(&sample_two));
    println!("{:?}", part_two(&input));
    Ok(())
}
