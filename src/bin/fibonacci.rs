use std::mem::swap;

struct Solution {}

impl Solution {
    pub fn fib(mut n: i32) -> i32 {
        let mut prev_2 = 0;
        let mut prev_1 = 1;

        if n == 0 {
            return prev_2;
        } else if n == 1 {
            return prev_1;
        }

        while n >= 2 {
            (prev_1, prev_2) = (prev_2, prev_1);
            prev_1 = prev_1 + prev_2;
            n = n - 1;
        }

        return prev_1;
    }

    pub fn fib_2(n: i32) -> i32 {
        let fibs = [0,
            1,
            1,
            2,
            3,
            5,
            8,
            13,
            21,
            34,
            55,
            89,
            144,
            233,
            377,
            610,
            987,
            1597,
            2584,
            4181,
            6765,
            10946,
            17711,
            28657,
            46368,
            75025,
            121393,
            196418,
            317811,
            514229,
            832040
        ];
        return fibs[n as usize];
    }
}

fn main() {
    for i in 0..=30 {
        println!("{}", Solution::fib(i));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(Solution::fib(2), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::fib(3), 2);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::fib(4), 3);
    }
}
