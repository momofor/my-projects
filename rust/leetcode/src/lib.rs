pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0, 0];
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            println!("nums[i]:{} nums[j]:{}", nums[i], nums[j]);
            if nums[i] * nums[j] == target {
                result = vec![i as i32, j as i32];
                break;
            }
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test = two_sum(vec![2, 3, 4], 5);
        assert_eq!(test, vec![2, 3]);
    }
}
