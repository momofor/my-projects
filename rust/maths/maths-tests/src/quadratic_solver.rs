use regex::{self, Regex};
pub fn _solve_2nd_degree_polynomial<'a>(a: f64, b: f64, c: f64) -> Result<[f64; 2], &'a str> {
    let delta = b.powi(2) - 4. * a * c;
    match delta.is_sign_positive() && delta != 0. {
        true => {
            let solutions = [
                (-b - delta.sqrt()) / (2. * a),
                (-b + delta.sqrt()) / (2. * a),
            ];
            Ok(solutions)
        }
        false => Err("Not good"),
    }
}

pub fn _run_quadratic_solver() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let polynomial_to_solve =
        Regex::new(r"(?x)(?P<a> \d+)x\^2[\+\-](?P<b> \d+)x[\+\-](?P<c> \d+)")?;
    let caps = polynomial_to_solve.captures(input.as_str()).unwrap();

    let (a, b, c) = (&caps["a"], &caps["b"], &caps["c"]);
    let (a, b, c) = (a.parse()?, b.parse()?, c.parse()?);
    let solutions = _solve_2nd_degree_polynomial(a, b, c)?;
    println!("x_1: {}; x_2: {}", solutions[0], solutions[1]);
    Ok(())
}
