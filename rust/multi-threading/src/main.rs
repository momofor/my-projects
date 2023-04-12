#![feature(portable_simd)]
#![feature(array_chunks)]
#![feature(slice_as_chunks)]

// use std::{
//     simd::{f32x16, SimdFloat, StdFloat},
// };

fn main() {
    let x: Vec<i32> = (0..10000000).collect();
    let y: Vec<i32> = (20000000..29999999).collect();
    println!("{}",easy_sum(&x, &y)) ;
}

// pub fn dot_prod_simd_5(a: &[f32], b: &[f32]) -> f32 {
//     a.array_chunks::<16>()
//         .map(|&a| f32x16::from_array(a))
//         .zip(b.array_chunks::<16>().map(|&b| f32x16::from_array(b)))
//         .fold(f32x16::splat(0.), |acc, (a, b)| a.mul_add(b, acc))
//         .reduce_sum()
// }
//

// completely auto-vectorized
fn easy_sum(a:&[i32],b:&[i32])->i32 {
    a.iter().zip(b).map(|(x,y)| x + y).sum()
}
