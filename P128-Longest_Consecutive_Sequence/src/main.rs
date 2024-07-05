use std::collections::HashSet;
fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        
        let mut longest_consecutive = 1;
        nums = nums.into_iter().collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();
        nums.sort();

        let mut index = 0;
        while index < nums.len() {
            let num = nums[index];

            let consecutive = Solution::compute(num, &nums[(index + 1)..]);

            if consecutive > longest_consecutive {
                longest_consecutive = consecutive;
            }

            index += consecutive as usize;
            
        }

        longest_consecutive
    }

    fn compute(num: i32, nums_slice: &[i32]) -> i32 {
        if nums_slice.is_empty() {
            1
        } else {
            if nums_slice[0] - num == 1 {
                1 + Solution::compute(nums_slice[0], &nums_slice[1..])
            } else {
                1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;


    #[test]
    fn test_1() {
        let input = vec![100,4,200,1,3,2];
        assert_eq!(4, Solution::longest_consecutive(input));
    }

    #[test]
    fn test_2() {
        let input = vec![0,3,7,2,5,8,4,6,0,1];
        assert_eq!(9, Solution::longest_consecutive(input));
    }
}