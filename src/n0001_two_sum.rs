/**
 * [1] Two Sum
 *
 * Given an array of integers, return indices of the two numbers such that they add up to a specific target.
 *
 * You may assume that each input would have exactly one solution, and you may not use the same element twice.
 *
 * Example:
 *
 *
 * Given nums = [2, 7, 11, 15], target = 9,
 *
 * Because nums[0] + nums[1] = 2 + 7 = 9,
 * return [0, 1].
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, num_i) in nums.iter().enumerate(){
            for (j, num_j) in nums.iter().enumerate() {
                if i==j {
                    continue
                }
                if num_i + num_j == target {
                    return vec![i as i32, j as i32]
                }
            }
        }
        unreachable!()
    }
}


pub fn two_sum2(nums: Vec<i32>, target: i32) -> usize {
    for (index, num) in nums.iter().enumerate() {
        println!("{}",num);
        return num;
    };
    unreachable!()
}


// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = two_sum2(vec![111,222],333);
    }

}
