struct Solution;

impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        nums.append(&mut nums.clone());
        nums
    }
}

fn main() {
    assert_eq!(Solution::add(1, 2), 3);
}
