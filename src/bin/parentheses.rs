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

    fn build_stack(stack: &mut Vec<u8>, num_open: i32, pairs: i32) -> Vec<Vec<u8>> {
        let size = stack.len() as i32;
        if size == pairs * 2 {
            return vec![stack.clone()];
        }

        let mut all_combinations = vec![];
        if (pairs * 2 - size) > num_open {
            // can push an open bracket if we have more spaces to go than brackets left to close
            stack.push(b'(');
            all_combinations.append(
                &mut Self::build_stack(stack, num_open + 1, pairs)
            );
            stack.pop();
        }
        if num_open > 0 {
            // push a closed bracket
            stack.push(b')');
            all_combinations.append(
                &mut Self::build_stack(stack, num_open - 1, pairs)
            );
            stack.pop();
        }

        return all_combinations;
    }

    pub fn generate_parenthesis_bkt(n: i32) -> Vec<String> {
        return Self::build_stack(&mut vec![], 0, n)
            .iter()
            .map(|chars| unsafe { std::str::from_utf8_unchecked(chars) }.to_string())
            .collect();
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parentheses_0() {
        assert_eq!(Solution::generate_parenthesis_bkt(0), vec![""]);
    }

    #[test]
    fn parentheses_1() {
        assert_eq!(Solution::generate_parenthesis_bkt(1), vec!["()"]);
    }

    #[test]
    fn parentheses_2() {
        assert_eq!(Solution::generate_parenthesis_bkt(2), vec!["(())", "()()"]);
    }

    #[test]
    fn parentheses_3() {
        assert_eq!(Solution::generate_parenthesis_bkt(3), vec!["((()))","(()())","(())()","()(())","()()()"]);
    }

    #[test]
    fn parentheses_4() {
        assert_eq!(Solution::generate_parenthesis_bkt(4), vec!["(((())))","((()()))","((())())","((()))()","(()(()))","(()()())","(()())()","(())(())","(())()()","()((()))","()(()())","()(())()","()()(())","()()()()"]);
    }
}
