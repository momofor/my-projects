#![feature(portable_simd)]
use std::simd::{f64x8, Simd};

use linear_algebra_lib::Matrix;

fn main() {
    let test_data = [1., 2., 3., 4., 5., 6., 7., 8.];
    let test_matrix = Matrix::new(4, 2, test_data);
    let testu_data = [2., 3., 4., 5., 6., 7., 8., 9.];
    let testu_matrix = Matrix::new(4, 2, testu_data);
    let (true_test_data, true_testu_data) =
        (Simd::from_array(test_data), Simd::from_array(testu_data));
    println!("{:?}", true_test_data + true_testu_data);
    println!("{:?}", test_matrix + testu_matrix);
}
