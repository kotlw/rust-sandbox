#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::with_capacity(asteroids.len());

        for a in asteroids.into_iter() {
            if stack.is_empty() || (stack.last().unwrap() < &0 && a < 0) || (a > 0) {
                stack.push(a);
            } else {
                while !stack.is_empty() && stack.last().unwrap() > &0 && stack.last().unwrap() < &-a {
                   stack.pop(); 
                }
                if !stack.is_empty() && stack.last().unwrap() == &-a {
                    stack.pop();
                } else if stack.is_empty() || (!stack.is_empty() && stack.last().unwrap() < &0) {
                    stack.push(a);
                }
            }

        }

        return stack;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(asteroids: Vec<i32>, expected_result: Vec<i32>) {
        let res = Solution::asteroid_collision(asteroids);
        assert_eq!(res, expected_result);
    }

    #[test]
    fn case_1() {
        run(vec![5, 10, -5], vec![5, 10]);
    }

    #[test]
    fn case_2() {
        run(vec![8, -8], vec![]);
    }

    #[test]
    fn case_3() {
        run(vec![10, 2, -5], vec![10]);
    }

    #[test]
    fn case_4() {
        run(vec![-2, 2, 1, -2], vec![-2]);
    }

    #[test]
    fn case_5() {
        run(vec![4, 3, -5, 9, -3], vec![-5, 9]);
    }
}
