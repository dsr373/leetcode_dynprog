struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // start at the end
        let mut current = (nums.len() as i32) - 1;

        nums.iter().enumerate().for_each(|(idx, num)| println!("{}: {}", idx, num));
        while current > 0 {
            let mut min_jumped_to = current;

            for jump in 1..=current {
                let jumped_to = (current - jump) as usize;
                if nums[jumped_to] >= (jump as i32) {
                    min_jumped_to = min_jumped_to.min(jumped_to as i32);
                }
            }

            if min_jumped_to == current {
                // can't go back any further
                return false;
            }
            current = min_jumped_to as i32;
        }

        return true;
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_jump_1() {
        assert_eq!(Solution::can_jump(vec![2,3,1,1,4]), true);
    }

    #[test]
    fn can_jump_2() {
        assert_eq!(Solution::can_jump(vec![3,2,1,0,4]), false);
    }

}
