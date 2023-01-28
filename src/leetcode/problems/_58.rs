#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn length_of_last_word(s: String) -> i32 {
        let mut res = 0;
        for c in s.chars().rev() {
            if c == ' ' {
                if res == 0 {
                    continue
                } else {
                    return res
                }
            }
            res += 1;
        }
        return res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: String, expected_result: i32) {
        let res = Solution::length_of_last_word(s);
        assert_eq!(res, expected_result);
    }

    #[test]
    fn case_1() {
        run("Hello World".to_owned(), 5);
    }

    #[test]
    fn case_2() {
        run("   fly me   to   the moon  ".to_owned(), 4);
    }

    #[test]
    fn case_3() {
        run("luffy is still joyboy".to_owned(), 6);
    }
}
