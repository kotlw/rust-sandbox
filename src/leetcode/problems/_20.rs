use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_valid(s: String) -> bool {
        let p = HashMap::from([
            (')', '('),
            (']', '['),
            ('}', '{'),
        ]);

        let mut stack = Vec::with_capacity(s.len());
        
        for ch in s.chars() {
            if p.values().any(|&c| c==ch) {
                stack.push(ch);
            } else if p.get(&ch) == stack.last() {
                stack.pop();            
            } else {
                return false
            }
        }

        return stack.is_empty()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: String, expected_result: bool) {
        let res = Solution::is_valid(s);
        assert_eq!(res, expected_result);
    }
    
    #[test]
    fn case_1() {
        run(String::from("()"), true);
    }

    #[test]
    fn case_2() {
        run(String::from("()[]{}"), true);
    }

    
    #[test]
    fn case_3() {
        run(String::from("(]"), false);
    }
}
