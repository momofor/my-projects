#![feature(portable_simd)]
#![feature(array_chunks)]
#![feature(slice_as_chunks)]

use std::simd::{f32x16, SimdFloat, StdFloat};

fn main() {
    let x: Vec<f32> = [0.5; 1000000].to_vec();
    let y: Vec<f32> = [2.0; 1000000].to_vec();
    let noice = dot_prod_simd_5(&x, &y);
    println!("{:?}", noice);
}

pub fn dot_prod_simd_5(a: &[f32], b: &[f32]) -> f32 {
    a.array_chunks::<16>()
        .map(|&a| f32x16::from_array(a))
        .zip(b.array_chunks::<16>().map(|&b| f32x16::from_array(b)))
        .fold(f32x16::splat(0.), |acc, (a, b)| a.mul_add(b, acc))
        .reduce_sum()
}
