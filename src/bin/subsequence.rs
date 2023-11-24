struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_idx = 0usize;

        let sub = s.as_bytes();
        let total = t.as_bytes();

        if sub.len() == 0 {
            return true;
        }

        for c in total {
            if c == &sub[s_idx] {
                s_idx += 1;
                if s_idx == sub.len() {
                    return true;
                }
            }
        }

        return false;
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsequence_1() {
        let s = "abc";
        let t = "ahbgdc";
        assert_eq!(Solution::is_subsequence(s.to_string(), t.to_string()), true);
    }

    #[test]
    fn subsequence_2() {
        let s = "axc";
        let t = "ahbgdc";
        assert_eq!(Solution::is_subsequence(s.to_string(), t.to_string()), false);
    }
}
