pub extern crate pip_core;

extern crate petgraph;

use pip_core::{utils, gen_game, Options};
use std::fmt::Debug;
use petgraph::{Graph, Direction, graph::NodeIndex};

pub fn run(ops: Options) {
    println!("{}", utils::yaml::to_pretty_string(&gen_game(ops)))
}

pub trait Solvable: Sized + Debug + PartialEq + Eq + Clone {
    fn next(&self) -> Vec<Self>;
    fn is_solved(&self) -> bool;
}

#[derive(Debug)]
struct SearchTree<S: Solvable> {
    search_graph: Graph<S, ()>,
    root_node: NodeIndex<u32>,
    current_node: NodeIndex<u32>,
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
        let mut search_graph = Graph::<S, ()>::new();
        let root_node = search_graph.add_node(s);
        let current_node = root_node;
        let search_state = SearchState::InProgress;

        // Sets up the search invariant (children expanded)
        let mut search_tree = Self { search_graph, root_node, current_node, search_state };
        search_tree.add_next(current_node);

        search_tree
    }

    pub fn dfs(&mut self) -> Option<Vec<S>> {
        use SearchState::*;

        self.search_state = self.check_solution();

        // Invariant: current_node is a valid node in search_graph, with its unsearched children expanded
        while self.search_state == InProgress {
            println!("{:#?}", self);
            match self.next_child() {
                Some(n) => {
                    self.add_next(n);
                    self.assign_current(n);
                    self.search_state = self.check_solution();
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

    fn add_next(&mut self, node: NodeIndex) {
        for child_state in self.search_graph.node_weight(node).unwrap().next() {
            let child_node = self.search_graph.add_node(child_state);
            self.search_graph.add_edge(node, child_node, ());
        }
    }

    fn check_solution(&self) -> SearchState {
        use SearchState::*;

        if self.search_graph.node_weight(self.current_node).unwrap().is_solved() {
            Solved
        } else {
            InProgress
        }
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
}