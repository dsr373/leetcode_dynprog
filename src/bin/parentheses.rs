struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let max_idx = n as usize;

        let mut combinations: Vec<Vec<String>> = vec![
            vec!["".to_string()],   // 0 pairs
            vec!["()".to_string()]  // 1 pair
        ];

        for i in 2usize..=max_idx {
            let mut combinations_with_i: Vec<String> = vec![];
            for combination in &combinations[i-1] {
                combinations_with_i.push("(".to_string() + combination + ")");
            }
            for combination in &combinations[i-1] {
                combinations_with_i.push(combination.clone() + "()");
                combinations_with_i.push("()".to_string() + combination);
            }
            combinations_with_i.pop();
            combinations.push(combinations_with_i);
        }

        for (idx, vector) in combinations.iter().enumerate() {
            println!("{}: {:?}", idx, vector);
        }

        return combinations[max_idx].iter().map(|s| s.clone()).collect();
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parentheses_0() {
        assert_eq!(Solution::generate_parenthesis(0), vec![""]);
    }

    #[test]
    fn parentheses_1() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
    }

    #[test]
    fn parentheses_3() {
        assert_eq!(Solution::generate_parenthesis(3), vec!["((()))","(()())","(())()","()(())","()()()"]);
    }

    #[test]
    fn parentheses_4() {
        assert_eq!(Solution::generate_parenthesis(4), vec!["(((())))","((()()))","((())())","((()))()","(()(()))","(()()())","(()())()","(())(())","(())()()","()((()))","()(()())","()(())()","()()(())","()()()()"]);
    }
}
