use itertools::Itertools;
use regex::{self, Regex};

fn my_inverse(num: i32) -> i32 {
    -num
}

fn my_composition_method(num: (i32, i32)) -> i32 {
    num.0 - num.1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let polynomial_to_solve =
        Regex::new(r"(?x) (?P<a> \d+)x\^2[\+\-](?P<b> \d+)x[\+\-](?P<c> \d+)")?;
    let caps = polynomial_to_solve.captures(input.as_str()).unwrap();

    let (a, b, c) = (&caps["a"], &caps["b"], &caps["c"]);
    let (a, b, c) = (a.parse()?, b.parse()?, c.parse()?);
    let solutions = solve_2nd_degree_polynomial(a, b, c)?;
    println!("x_1: {}; x_2: {}", solutions[0], solutions[1]);
    let my_group = Group::new(
        "My_group".to_string(),
        0,
        my_inverse,
        my_composition_method,
        vec![1, 2, 3, 4, 5, 5],
    );
    let (commutativity, inversability) = my_group.check_validity();
    println!("com:{commutativity}, inver:{inversability}");

    Ok(())
}
struct Group {
    name: String,
    neuteural: i32,
    elements: Vec<i32>,
    inverse_method: fn(i32) -> i32,
    composition_method: fn((i32, i32)) -> i32,
}

impl Group {
    fn inverse(&self, num: i32) -> i32 {
        (self.inverse_method)(num)
    }
    fn compose(&self, nums: (i32, i32)) -> i32 {
        (self.composition_method)(nums)
    }
    fn check_validity(&self) -> (bool, bool) {
        let elements = &self.elements;
        let mut is_commutative = true;
        let mut is_inversable = true;
        for (a, b) in elements.iter().tuple_windows() {
            if self.compose((*a, *b)) == self.compose((*b, *a)) {
                println!("{a}, {b} are commutative");
            } else {
                println!("{a}, {b} are not commutative");
                is_commutative = false;
                break;
            }
        }

        for element in elements {
            if self.compose((self.inverse(*element), *element)) == self.neuteural {
                println!("{element} is inversable");
            } else {
                is_inversable = false;
                println!("{element} isn't inversable");
                break;
            }
        }
        (is_commutative, is_inversable)
    }
    fn new(
        name: String,
        neuteural: i32,
        inverse_method: fn(i32) -> i32,
        composition_method: fn((i32, i32)) -> i32,
        elements: Vec<i32>,
    ) -> Group {
        let new_group = Group {
            name,
            neuteural,
            composition_method,
            inverse_method,
            elements,
        };
        new_group
    }
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
