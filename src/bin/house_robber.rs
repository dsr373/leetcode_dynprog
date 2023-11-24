struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return nums[0].max(nums[1]);
        }

        let mut robbed_1_and_3_ago = nums[0] + nums[2];
        let mut robbed_2_ago = nums[1];
        let mut robbed_3_ago_and_skipped = nums[0];

        let mut rob_last = robbed_2_ago;
        let mut rob_current = robbed_1_and_3_ago;

        for i in 3..nums.len() {
            rob_last = robbed_1_and_3_ago.max(robbed_2_ago);
            rob_current = (robbed_2_ago + nums[i]).max(robbed_3_ago_and_skipped + nums[i]);

            robbed_3_ago_and_skipped = robbed_2_ago;
            robbed_2_ago = rob_last;
            robbed_1_and_3_ago = rob_current;
        }

        return rob_last.max(rob_current);
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn robber_base() {
        assert_eq!(Solution::rob(vec![12]), 12);
    }

    #[test]
    fn robber_base_2el() {
        assert_eq!(Solution::rob(vec![7, 12]), 12);
    }

    #[test]
    fn robber_3el_middle_is_better() {
        assert_eq!(Solution::rob(vec![1, 10, 1]), 10);
    }

    #[test]
    fn robber_3el_sum_is_better() {
        assert_eq!(Solution::rob(vec![7, 10, 12]), 19);
    }

    #[test]
    fn robber_1() {
        assert_eq!(Solution::rob(vec![1,2,3,1]), 4);
    }

    #[test]
    fn robber_2() {
        assert_eq!(Solution::rob(vec![2,7,9,3,1]), 12);
    }

    #[test]
    fn robber_skip_2() {
        assert_eq!(Solution::rob(vec![2,1,1,2]), 4);
    }
}
