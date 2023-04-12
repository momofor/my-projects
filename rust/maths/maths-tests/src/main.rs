mod groups;
// mod quadratic_solver;
use groups::Group;

#[inline]
fn my_inverse(num: &i32) -> i32 {
    -num
}

#[inline]
fn my_composition_method(num: (&i32, &i32)) -> i32 {
    num.0 + num.1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_data: Vec<i32> = (0..1000020).collect();
    const DATA_SIZE: usize = 1000020;
    assert_eq!(test_data.len(), DATA_SIZE);
    let test_data: [i32; DATA_SIZE] = test_data.try_into().unwrap();
    let my_group = Group::new(
        "My Group".to_string(),
        0,
        my_inverse,
        my_composition_method,
        test_data,
    );
    let is_commutative = my_group.simd_is_commutative();
    println!("{is_commutative}");
    Ok(())
}
