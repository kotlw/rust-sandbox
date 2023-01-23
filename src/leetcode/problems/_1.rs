use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut find = HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            match find.get(v) {
                Some(&s) => return vec![s as i32, i as i32],
                None => {
                    find.insert(target - v, i);
                }
            }
        }

        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(nums: Vec<i32>, target: i32, expected_result: Vec<i32>) {
        let res = Solution::two_sum(nums, target);
        assert_eq!(res, expected_result);
    }
    
    #[test]
    fn case_1() {
        run(vec![2,7,11,15], 9, vec![0,1]);
    }

    #[test]
    fn case_2() {
        run(vec![3,2,4], 6, vec![1,2]);
    }

    #[test]
    fn case_3() {
        run(vec![3,3], 6, vec![0,1]);
    }
}