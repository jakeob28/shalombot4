use crate::digits::operation::Operation;
use game_state::GameState;

use petgraph::data::Build;
use petgraph::graphmap::DiGraphMap;

mod game_state;
mod operation;

pub(crate) struct DigitsSolver<const N: usize> {
    state_graph: DiGraphMap<GameState<N>, Vec<Operation>>,
    initial_state: GameState<N>,
    target: usize,
    best_solution_size: usize,
    target_states: Vec<GameState<N>>,
}

impl<const N: usize> DigitsSolver<N> {
    pub fn solve(target: usize, digits: Vec<usize>) -> DigitsSolver<N> {
        let mut solver = DigitsSolver {
            state_graph: DiGraphMap::new(),
            initial_state: GameState::new(digits),
            target,
            best_solution_size: 1,
            target_states: Vec::new(),
        };

        solver.solve_state(solver.initial_state);

        solver
    }

    pub fn get_solutions(&self) -> Vec<Vec<Operation>> {
        let mut solutions: Vec<Vec<Operation>> = Vec::new();

        for target_state in &self.target_states {
            let path = petgraph::algo::astar(
                &self.state_graph,
                self.initial_state,
                |node| *target_state == node,
                |_| 1,
                |_| 0,
            );
            if let Some(path) = path {
                let mut solution: Vec<Operation> = Vec::with_capacity(path.1.len());
                for i in 1..path.1.len() {
                    let edge = &self.state_graph.edge_weight(path.1[i - 1], path.1[i]);
                    solution.push(edge.unwrap()[0]);
                }
                solutions.push(solution);
            }
        }
        solutions
    }

    fn solve_state(&mut self, state: GameState<N>) {
        for (next_state, mut operations) in state.next_states() {
            if self.state_graph.contains_node(next_state) {
                continue;
            }

            self.state_graph.add_node(next_state);

            if let Some(&existing_ops) = self.state_graph.edge_weight(state, next_state).as_ref() {
                let mut new_ops = existing_ops.clone();
                new_ops.append(&mut operations);
                self.state_graph.update_edge(state, next_state, new_ops);
            } else {
                self.state_graph.add_edge(state, next_state, operations);
            }
            if next_state.digits.contains(&self.target) {
                if self.best_solution_size < next_state.num_digits {
                    self.best_solution_size = next_state.num_digits;
                    self.target_states.clear();
                }
                self.target_states.push(next_state);
                continue;
            }
            if next_state.num_digits > self.best_solution_size {
                self.solve_state(next_state);
            }
        }
    }
}
