mod groups;
mod quadratic_solver;
use groups::Group;

fn my_inverse(num: &i32) -> i32 {
    -num
}

fn my_composition_method(num: (&i32, &i32)) -> i32 {
    num.0 + num.1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_vec = vec![1, 2, 3, 4, 5, 5];
    let my_group = Group::new(
        "My Group".to_string(),
        0,
        my_inverse,
        my_composition_method,
        test_vec,
    );
    let (commutativity, inversability) = my_group.check_validity();
    println!("com:{commutativity}, inver:{inversability}");
    println!("{:?}", my_group);
    Ok(())
}
