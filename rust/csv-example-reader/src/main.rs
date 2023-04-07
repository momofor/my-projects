use std::fs::read_to_string;

#[derive(Debug, serde::Deserialize, Eq, PartialEq)]
struct Model {
    student_name: String,
    class: u8,
    grade: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_to_string("./test.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(data.as_bytes());

    for result in rdr.deserialize::<Model>() {
        let record: Model = result?;
        println!(
            "student name is: {}, their class is: {} and their grade is: {}",
            record.student_name, record.class, record.grade
        );
    }
    Ok(())
}
