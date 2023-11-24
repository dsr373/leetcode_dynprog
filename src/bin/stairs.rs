struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut ways = Vec::with_capacity(n as usize);
        ways.extend([0, 1, 2]);

        let vec_size: usize = n as usize;
        for i in 3usize..=vec_size {
            ways.push(
                ways[1] * ways[i - 1] + ways[i - 2]
            );
        }

        return ways[n as usize];
    }

    pub fn paths_up_to_n(n: i32) -> Vec<Vec<u8>> {
        let mut paths: Vec<Vec<Vec<u8>>> = vec![
            vec![],
            vec![vec![1]],
            vec![vec![1, 1], vec![2]]
        ];

        let vec_size: usize = n.try_into().unwrap();
        for i in 3usize..=vec_size {
            // all the paths until 1 before + 1
            let paths_with_1 = paths[i - 1].iter().map(|path| {
                let mut new_path = path.clone();
                new_path.push(1u8);
                return new_path;
            }).collect::<Vec<Vec<u8>>>();

            // all paths until 2 before + 2
            let paths_with_2 = paths[i - 2].iter().map(|path| {
                let mut new_path = path.clone();
                new_path.push(2u8);
                return new_path;
            }).collect::<Vec<Vec<u8>>>();

            paths.push([paths_with_1, paths_with_2].concat());
        }

        for i in 1..=vec_size {
            println!("N = {} has {} ways to climb. Paths:\n{:?}", i, paths[i].len(), paths[i]);
        }

        return paths[n as usize].clone();
    }
}

fn main() {
    let n = 10;
    println!("N = {} has {} ways to climb. Paths:\n{:?}", n, Solution::climb_stairs(n), Solution::paths_up_to_n(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
