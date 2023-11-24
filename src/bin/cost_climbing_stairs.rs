struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut prev_1 = 0;
        let mut prev_2 = 0;

        if cost.len() == 0 {
            return 0;
        } else if cost.len() == 1 {
            return 0;
        }

        let mut current = 0;
        for i in 2..=cost.len() {
            let cost_from_1_before = prev_1 + cost[i-1];
            let cost_from_2_before = prev_2 + cost[i-2];
            current = cost_from_1_before.min(cost_from_2_before);
            prev_2 = prev_1;
            prev_1 = current;
        }

        return current;
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![1,100,1,1,1,100,1,1,100,1]), 6);
    }
}
