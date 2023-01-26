#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn jump(nums: Vec<i32>) -> i32 {
        let (mut res, mut cur_end, mut cur_far) = (0, 0, 0);

        for i in 0..nums.len() - 1 {
            cur_far = std::cmp::max(cur_far, nums[i] + i as i32);
            
            if cur_end == i as i32 {
                res += 1;
                cur_end = cur_far;
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(nums: Vec<i32>, expected_result: i32) {
        let res = Solution::jump(nums);
        assert_eq!(res, expected_result);
    }

    #[test]
    fn case_1() {
        run(vec![2, 3, 1, 1, 4], 2);
    }

    #[test]
    fn case_2() {
        run(vec![2, 3, 0, 1, 4], 2);
    }

    #[test]
    fn case_3() {
        run(vec![1], 0);
    }
}
