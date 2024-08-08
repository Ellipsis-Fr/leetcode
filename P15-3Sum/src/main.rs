use std::collections::{HashSet, VecDeque};



fn main() {
    let a = Solution::three_sum(vec![0,0,0]);
    dbg!(a);
    // assert_eq!(vec![1, 2], Solution::three_sum(vec![2,7,11,15]));
}

struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut three_sum: HashSet<Vec<i32>> = HashSet::new();
        nums.sort();

        let mut i = 0usize;
        let mut j = nums.len() - 1;
        let mut j_maxs = VecDeque::from([j]);
        let mut i_moved = false;
        let mut j_moved = false;
        
        loop {
            if i > nums.len() - 1 {
                break;
            }

            let num_i = &nums[i];
            

            if i_moved {
                if num_i == &nums[i - 1] {
                    i += 1;
                    continue;
                }

                i_moved = false;

                loop {
                    let j_max = j_maxs.pop_front().unwrap();
                    if *num_i <= 0 - (*num_i + *&nums[j_max]) {
                        j = j_max;
                        j_maxs.push_front(j_max);
                        break;
                    } else {
                        if j_max - j > 1 {
                            continue;
                        } else {
                            j_maxs.push_back(j);
                            break;
                        }
                    }

                }
            }

            if i + 1 >= j {
                break;
            }

            let num_j = &nums[j];

            if j_moved {
                if num_j == &nums[j + 1] {
                    j -= 1;
                    continue;
                } else {
                    j_moved = false;
                    
                    let j_max = j_maxs.get(0).unwrap();
                    if !j_maxs.contains(&(j + 1)) && *j_max > j + 1 {
                        j_maxs.push_back(j + 1);
                    }
                }
            }
            
            let value_to_find = 0 - (num_i + num_j);
            
            let shift = if (value_to_find - num_i).abs() <= (value_to_find - num_j).abs() {
                1
            } else {
                -1
            };

            let mut k = if shift > 0 {
                i + shift as usize
            } else  {
                (j as i32 + shift) as usize
            };

            loop {
                let num_k = &nums[k];
                
                if value_to_find == *num_k {
                    three_sum.insert(vec![*num_i, *num_j, *num_k]);
                } else if value_to_find < *num_k && shift > 0 {
                    j -= 1;
                    j_moved = true;
                    break;
                } else if value_to_find > *num_k && shift < 0 {
                    if *num_i < 0 && *num_j > 0 && num_i.abs() - num_j < (num_i.abs() / 2) {
                        j -= 1;
                        j_moved = true;
                        break;
                    }
                    i += 1;
                    i_moved = true;
                    break;
                }
                
                k = (k as i32 + shift) as usize;
                
                if i == k || j == k {
                    if shift < 0 || (value_to_find - num_i).abs() == (value_to_find - num_j).abs() {
                        i += 1;
                        i_moved = true;
                    } else  {
                        j -= 1;
                    };

                    break;
                }
            }
        }


        three_sum.into_iter().collect()
    }
}