pub extern crate pip_core;

extern crate petgraph;

use pip_core::{utils, gen_game, Options};
use std::fmt::Debug;
use petgraph::{graphmap::DiGraphMap, Direction};
use std::{hash::Hash};

pub fn run(ops: Options) {
    println!("{}", utils::yaml::to_pretty_string(&gen_game(ops)))
}

pub trait Solvable: PartialEq + Eq + Ord + Copy + Hash + Debug {
    fn next(&self) -> Vec<Self>;
    fn is_solved(&self) -> bool;
}

#[derive(Debug)]
struct SearchTree<S: Solvable> {
    search_graph: DiGraphMap<S, ()>,
    initial: S,
    current: S,
    state_limit: Option<u64>,
    states_visited: u64,
    search_state: SearchState,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum SearchState {
    InProgress,
    Solved,
    NoSolution,
}

impl<S: Solvable> SearchTree<S> {
    pub fn new(s: S) -> Self {
        let mut search_graph = DiGraphMap::new();
        search_graph.add_node(s);

        // Sets up the search invariant (children expanded)
        let mut search_tree = Self {
            search_graph,
            initial: s,
            current: s,
            state_limit: None,
            states_visited: 0,
            search_state: SearchState::InProgress,
        };
        search_tree.add_next(s);

        search_tree
    }

    pub fn set_depth_limit(&mut self, limit: u64) {
        self.state_limit = Some(limit)
    }

    pub fn dfs(&mut self) -> Option<Vec<S>> {
        use SearchState::*;

        self.search_state = self.check_solution();

        // Invariant: current_node is a valid node in search_graph, with its unsearched children expanded
        while self.continue_search() {
            println!("{:#?}", self);
            match self.next_child() {
                Some(s) => {
                    self.add_next(s);
                    self.current = s;
                    self.search_state = self.check_solution();
                },
                None => {
                    let old_current = self.current;
                    match self.parent() {
                        Some(parent) => {
                            self.current = parent;
                            self.remove(old_current, parent);
                        },
                        None => self.search_state = NoSolution,
                    }
                },
            }
        }

        if self.search_state == Solved {
            Some(self.search_path())
        } else {
            None
        }
    }

    fn continue_search(&self) -> bool {
        self.search_state == SearchState::InProgress
            && self.state_limit.map_or(true, |l| l > self.states_visited)
    }

    fn add_next(&mut self, s: S) {
        for child in s.next() {
            if !self.search_graph.contains_node(child) {
                self.search_graph.add_node(child);
                self.search_graph.add_edge(s, child, ());
            }
        }
    }

    fn check_solution(&mut self) -> SearchState {
        use SearchState::*;

        self.states_visited += 1;

        if self.current.is_solved() {
            Solved
        } else {
            InProgress
        }
    }

    fn next_child(&self) -> Option<S> {
        self.search_graph.neighbors_directed(self.current, Direction::Outgoing).into_iter().next()
    }

    fn parent(&self) -> Option<S> {
        self.search_graph.neighbors_directed(self.current, Direction::Incoming).into_iter().next()
    }

    fn remove(&mut self, s: S, parent: S) {
        self.search_graph.remove_edge(parent, s);
        self.search_graph.remove_node(s);
    }

    fn search_path(&self) -> Vec<S> {
        let mut states = Vec::new();
        let mut s = self.initial;

        loop {
            states.push(s);
            if s == self.current {
                break
            }
            s = self.search_graph.neighbors_directed(s, Direction::Outgoing).into_iter().next().unwrap()
        }

        states
    }
}

#[cfg(test)]
mod tests {
    use super::{*};

    impl Solvable for u64 {
        fn next(&self) -> Vec<Self> {
            if *self < 72 {
                vec![self * 2, self * 3, self * 1]
            } else {
                vec![]
            }
        }
        fn is_solved(&self) -> bool {
            *self == 72
        }
    }

    fn setup_tree(init: u64) -> SearchTree<u64> {
        let mut t = SearchTree::new(init);
        t.set_depth_limit(10000);
        t
    }

    #[test]
    fn dfs_num_test_1() {
        let mut t = setup_tree(1);

        assert_eq!(Some(vec![1, 2, 4, 8, 24, 72]), t.dfs());
    }

    #[test]
    fn dfs_num_test_2() {
        let mut t = setup_tree(2);

        assert_eq!(Some(vec![2, 4, 8, 24, 72]), t.dfs());
    }

    #[test]
    fn dfs_num_test_3() {
        let mut t = setup_tree(3);

        assert_eq!(Some(vec![3, 6, 12, 24, 72]), t.dfs());
    }

    #[test]
    fn dfs_num_test_4() {
        let mut t = setup_tree(4);

        assert_eq!(Some(vec![4, 8, 24, 72]), t.dfs());
    }

    #[test]
    fn dfs_num_test_5() {
        let mut t = setup_tree(5);

        assert_eq!(None, t.dfs());
    }

    #[test]
    fn dfs_num_test_6() {
        let mut t = setup_tree(6);

        assert_eq!(Some(vec![6, 12, 24, 72]), t.dfs());
    }

    #[test]
    fn dfs_num_test_7() {
        let mut t = setup_tree(7);

        assert_eq!(None, t.dfs());
    }

    #[test]
    fn one_state_visited() {
        let mut t = setup_tree(72);
        t.dfs();

        assert_eq!(1, t.states_visited);
    }

    #[test]
    fn two_states_visited() {
        let mut t = setup_tree(36);
        t.dfs();

        assert_eq!(2, t.states_visited);
    }

    #[test]
    fn eight_states_visited() {
        let mut t = setup_tree(3);
        t.dfs();

        assert_eq!(8, t.states_visited);
    }

    #[test]
    fn search_state_limit_cutoff() {
        let mut t = SearchTree::new(3);
        t.set_depth_limit(7);
        t.dfs();

        assert_eq!(SearchState::InProgress, t.search_state);
    }

    #[test]
    fn search_state_limit_not_cutoff() {
        let mut t = SearchTree::new(3);
        t.set_depth_limit(8);
        t.dfs();

        assert_eq!(SearchState::Solved, t.search_state);
    }
}