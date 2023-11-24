use std::mem::swap;

struct Solution {}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut prev_3 = 0;
        let mut prev_2 = 1;
        let mut prev_1 = 1;

        if n == 0 {
            return prev_3;
        } else if n == 1 {
            return prev_2;
        } else if n == 2 {
            return prev_1;
        }

        let mut new = 0;
        for _ in 3..=n {
            new = prev_1 + prev_2 + prev_3;
            println!("{} = {} + {} + {}", new, prev_1, prev_2, prev_3);
            prev_3 = prev_2;
            prev_2 = prev_1;
            prev_1 = new;
        }

        return prev_1;
    }
}

fn main() {
    for i in 0..=30 {
        println!("{}", Solution::tribonacci(i));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        assert_eq!(Solution::tribonacci(4), 4);
    }

    #[test]
    fn test_25() {
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}
