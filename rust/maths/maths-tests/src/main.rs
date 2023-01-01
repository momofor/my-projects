use regex::{self, Regex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let polynomial_to_solve = Regex::new(r"(?x) (?P<a> \d)x\^2[\+\-](?P<b> \d)x[\+\-](?P<c> \d)")?;
    let caps = polynomial_to_solve.captures(input.as_str()).unwrap();

    let (a, b, c) = (&caps["a"], &caps["b"], &caps["c"]);
    let (a, b, c) = (a.parse()?, b.parse()?, c.parse()?);
    let solutions = solve_2nd_degree_polynomial(a, b, c)?;
    println!("x_1: {}; x_2: {}", solutions[0], solutions[1]);

    Ok(())
}

fn solve_2nd_degree_polynomial(a: f64, b: f64, c: f64) -> Result<Vec<f64>, String> {
    let delta = b.powi(2) - 4. * a * c;
    match delta.is_sign_positive() && delta != 0. {
        true => {
            let mut solutions: Vec<f64> = vec![];
            solutions.push((-b - delta.sqrt()) / (2. * a));
            solutions.push((-b + delta.sqrt()) / (2. * a));
            return Ok(solutions);
        }
        false => {
            return Err("Not good".to_string());
        }
    }
}
