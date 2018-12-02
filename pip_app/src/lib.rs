pub extern crate pip_core;

use pip_core::{utils, gen_game, Options};
use std::fmt::Debug;

pub fn run(ops: Options) {
    println!("{}", utils::yaml::to_pretty_string(&gen_game(ops)))
}

pub trait Solvable: Sized + Debug {
    fn next(&self) -> Vec<Self>;
    fn is_solved(&self) -> bool;
}

pub fn dfs<S: Solvable>(node: S) -> bool {
    let mut frontier = vec![node];

    while let Some(search_node) = frontier.pop() {
        println!("{:?} --- {:?}", search_node, frontier);
        if search_node.is_solved() {
            return true
        } else {
            frontier.append(&mut search_node.next());
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfs_multiple_test() {
        impl Solvable for u64 {
            fn next(&self) -> Vec<Self> {
                if *self < 72 {
                    vec![self * 2, self * 3]
                } else {
                    vec![]
                }
            }
            fn is_solved(&self) -> bool {
                *self == 72
            }
        }
        assert!(dfs(1));
        assert!(dfs(2));
        assert!(dfs(3));
        assert!(dfs(4));
        assert!(!dfs(5));
        assert!(dfs(6));
        assert!(!dfs(7));
    }
}