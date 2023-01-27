#[allow(dead_code)]
struct Solution;

impl Solution {
    fn backtrack(candidates: &[i32], curr: &mut Vec<i32>, target: i32, curr_sum: i32, res: &mut Vec<Vec<i32>>) {
        if target == curr_sum {
            res.push(curr.to_owned());
            return;
        } else if target < curr_sum {
            return;
        } else {
           for (i, &c) in candidates.iter().enumerate() {
                curr.push(c);
                Self::backtrack(&candidates[i..], curr, target, curr_sum + c, res);
                curr.pop();
           } 
        }
    }

    #[allow(dead_code)]
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut curr = vec![];
        let mut res = vec![];
        Self::backtrack(&candidates, &mut curr, target, 0,&mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(candidates: Vec<i32>, target: i32, expected_result: Vec<Vec<i32>>) {
        let res = Solution::combination_sum(candidates, target);
        assert_eq!(res, expected_result);
    }

    #[test]
    fn case_1() {
        run(vec![2, 3, 6, 7], 7, vec![vec![2, 2, 3], vec![7]]);
    }

    #[test]
    fn case_2() {
        run(
            vec![2, 3, 5],
            8,
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
        );
    }

    #[test]
    fn case_3() {
        run(vec![2], 1, vec![]);
    }

    #[test]
    fn case_4() {
        run(vec![4,2,8], 8, vec![vec![4,4], vec![4,2,2], vec![2,2,2,2], vec![8]]);
    }
}
