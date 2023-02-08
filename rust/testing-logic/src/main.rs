fn main() {
    // two statements
    let p = false;
    let q = true;
    // exclusive or (xor)
    if (p | q) & !(p & q) {
        println!("xor");
    }
    // and
    if p & q {
        println!("and");
    }
    // or
    if p | q {
        println!("or");
    }
    // not
    if !p {
        println!("not p");
    }
    if !q {
        println!("not q");
    }
    // println!("{:?}", two_sum(vec![3, 2, 4], 6));
    // let test_string = "IV".to_string();
    // println!("{}", roman_to_int(test_string));
    let test_palindrome = Some(Box::new(ListNode::new(1)));
    println!("{}", is_palindrome(test_palindrome));
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    false
}

pub fn roman_to_int(s: String) -> i32 {
    let mut result = 0;
    let mut previous_char: Option<char> = None;
    for character in s.chars() {
        match (character, previous_char) {
            ('I', _) => result += 1,
            ('V', Some('I')) => result += 4 - 1,
            ('V', _) => result += 5,
            ('X', Some('I')) => result += 9 - 1,
            ('X', _) => result += 10,
            ('L', Some('X')) => result += 40 - 10,
            ('L', _) => result += 50,
            ('C', Some('X')) => result += 90 - 10,
            ('C', _) => result += 100,
            ('D', Some('C')) => result += 400 - 100,
            ('D', _) => result += 500,
            ('M', Some('C')) => result += 900 - 100,
            ('M', _) => result += 1000,
            _ => (),
        }
        previous_char = Some(character);
    }
    return result;
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0, 0];
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            println!("{}", nums[i] + nums[j]);
            if nums[i] + nums[j] == target {
                println!("found i:{} j:{}", i, j);
                println!("{}", nums[i] + nums[j]);
                result = vec![i as i32, j as i32];
                break;
            }
        }
    }
    return result;
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
