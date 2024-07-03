use std::collections::HashMap;

fn main() {
}

struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut cache = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            if let Some(value) = cache.get(num) {
                result.push(*value);
            } else {
                let mut nums_without_self = nums.clone();
                nums_without_self.remove(index);
                if nums_without_self.contains(&0) {
                    result.push(0);
                } else {
                    let product = nums_without_self.iter().product();
                    result.push(product);
                    cache.insert(num, product);
                }
            }
        }


        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;


    #[test]
    fn test_1() {
        let input = vec![1,2,3,4];
        let output = Solution::product_except_self(input);

        assert_eq!(vec![24,12,8,6], output);
    }

    #[test]
    fn test_2() {
        let input = vec![-1,1,0,-3,3];
        let output = Solution::product_except_self(input);

        assert_eq!(vec![0,0,9,0,0], output);
    }
}
