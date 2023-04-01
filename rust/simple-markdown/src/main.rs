use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = std::env::args().nth(1).unwrap();

    let contents = read_to_string(file_name)?;
    println!("{contents}");
    Ok(())
}
