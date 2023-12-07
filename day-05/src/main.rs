use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {

    let sample = read_to_string("sample")?;
    // let input = read_to_string("input")?;

    println!("Sample: {}", sample);
    Ok(())
}
