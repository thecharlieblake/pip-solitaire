pub extern crate pip_core;

extern crate petgraph;

use pip_core::{utils, gen_game, Options};
use std::fmt::Debug;
use petgraph::{Graph, Direction, graph::NodeIndex};
use std::{hash::Hash, collections::HashSet};

pub fn run(ops: Options) {
    println!("{}", utils::yaml::to_pretty_string(&gen_game(ops)))
}

pub trait Solvable: Sized + Debug + PartialEq + Eq + Clone + Hash {
    fn next(&self) -> Vec<Self>;
    fn is_solved(&self) -> bool;
}

#[derive(Debug)]
struct SearchTree<S: Solvable> {
    search_graph: Graph<S, ()>,
    root_node: NodeIndex<u32>,
    current_node: NodeIndex<u32>,
    state_limit: Option<u64>,
    states_visited: u64,
    search_state: SearchState,
    //transposition_table: HashSet<S>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum SearchState {
    InProgress,
    Solved,
    NoSolution,
}

impl<S: Solvable> SearchTree<S> {
    pub fn new(s: S) -> Self {
        let mut search_graph = Graph::<S, ()>::new();
        let root_node = search_graph.add_node(s);

        // Sets up the search invariant (children expanded)
        let mut search_tree = Self {
            search_graph,
            root_node,
            current_node: root_node,
            state_limit: None,
            states_visited: 0,
            search_state: SearchState::InProgress,
        };
        search_tree.add_next(root_node);

        search_tree
    }

    pub fn set_depth_limit(&mut self, limit: u64) {
        self.state_limit = Some(limit)
    }

    pub fn dfs(&mut self) -> Option<Vec<S>> {
        use SearchState::*;

        self.search_state = self.check_solution();

        // Invariant: current_node is a valid node in search_graph, with its unsearched children expanded,
        // and with a state that beed added to the transposition table
        while self.continue_search() {
            //println!("{:#?}", self);
            match self.next_child() {
                Some(n) => {
                    if self.cache(n) {
                        self.add_next(n);
                        self.assign_current(n);
                        self.search_state = self.check_solution();
                    } else {
                        self.remove(n);
                    }
                },
                None => {
                    let old_node_ref = self.current_node.clone();
                    match self.parent() {
                        Some(n) => self.assign_current(n),
                        None => self.search_state = NoSolution,
                    }
                    self.remove(old_node_ref)
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

    fn add_next(&mut self, node: NodeIndex) {
        for child_state in self.search_graph.node_weight(node).unwrap().next() {
            let child_node = self.search_graph.add_node(child_state);
            self.search_graph.add_edge(node, child_node, ());
        }
    }

    fn check_solution(&mut self) -> SearchState {
        use SearchState::*;

        self.states_visited += 1;

        if self.search_graph.node_weight(self.current_node).unwrap().is_solved() {
            Solved
        } else {
            InProgress
        }
    }

    fn cache(&mut self, n: NodeIndex) -> bool {
        true//self.transposition_table.insert(self.search_graph.node_weight(n).unwrap())
    }

    fn next_child(&self) -> Option<NodeIndex> {
        self.search_graph.neighbors_directed(self.current_node, Direction::Outgoing).into_iter().next()
    }

    fn parent(&self) -> Option<NodeIndex> {
        self.search_graph.neighbors_directed(self.current_node, Direction::Incoming).into_iter().next()
    }

    fn assign_current(&mut self, node: NodeIndex) {
        self.current_node = node.clone();
    }

    fn remove(&mut self, node: NodeIndex) {
        self.search_graph.remove_node(node);
    }

    fn search_path(&self) -> Vec<S> {
        let mut states = Vec::new();
        let mut n = self.root_node;

        loop {
            states.push(self.search_graph.node_weight(n).unwrap().clone());
            if n == self.current_node {
                break
            }
            n = self.search_graph.neighbors_directed(n, Direction::Outgoing).into_iter()
                .next().unwrap().clone()
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
                vec![self * 3, self * 2]
            } else {
                vec![]
            }
        }
        fn is_solved(&self) -> bool {
            *self == 72
        }
    }

    #[test]
    fn dfs_num_test_1() {
        assert_eq!(
            Some(vec![1, 2, 4, 8, 24, 72]),
            SearchTree::new(1).dfs(),
        );
    }

    #[test]
    fn dfs_num_test_2() {
        assert_eq!(
            Some(vec![2, 4, 8, 24, 72]),
            SearchTree::new(2).dfs(),
        );
    }

    #[test]
    fn dfs_num_test_3() {
        assert_eq!(
            Some(vec![3, 6, 12, 24, 72]),
            SearchTree::new(3).dfs(),
        );
    }

    #[test]
    fn dfs_num_test_4() {
        assert_eq!(
            Some(vec![4, 8, 24, 72]),
            SearchTree::new(4).dfs(),
        );
    }

    #[test]
    fn dfs_num_test_5() {
        assert_eq!(
            None,
            SearchTree::new(5).dfs(),
        );
    }

    #[test]
    fn dfs_num_test_6() {
        assert_eq!(
            Some(vec![6, 12, 24, 72]),
            SearchTree::new(6).dfs(),
        );
    }

    #[test]
    fn dfs_num_test_7() {
        assert_eq!(
            None,
            SearchTree::new(7).dfs(),
        );
    }

    #[test]
    fn one_state_visited() {
        let mut s = SearchTree::new(72);
        s.dfs();
        assert_eq!(
            1,
            s.states_visited,
        );
    }

    #[test]
    fn two_states_visited() {
        let mut s = SearchTree::new(36);
        s.dfs();
        assert_eq!(
            2,
            s.states_visited,
        );
    }

    #[test]
    fn ten_states_visited() {
        let mut s = SearchTree::new(3);
        s.dfs();
        assert_eq!(
            8,
            s.states_visited,
        );
    }

    #[test]
    fn search_state_limit_cutoff() {
        let mut s = SearchTree::new(3);
        s.set_depth_limit(7);
        s.dfs();
        assert_eq!(
            SearchState::InProgress,
            s.search_state,

        );
    }

    #[test]
    fn search_state_limit_not_cutoff() {
        let mut s = SearchTree::new(3);
        s.set_depth_limit(8);
        s.dfs();
        assert_eq!(
            SearchState::Solved,
            s.search_state,

        );
    }

    // TODO: cache loop test, cache implementation
}