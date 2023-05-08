use crate::digits::operation::Operation;
use crate::digits::operation::OperationType::{Add, Divide, Multiply, Subtract};
use std::cmp::{max, min};
use std::collections::HashMap;

/// Represents a potential game state. Has a parameter N as a capacity
#[derive(Hash, Eq, PartialEq, Debug, Ord, PartialOrd, Clone, Copy)]
pub struct GameState<const N: usize> {
    pub(crate) num_digits: usize,
    pub(crate) digits: [usize; N],
}

impl<const N: usize> GameState<N> {
    pub(crate) fn new(mut digits_vec: Vec<usize>) -> GameState<N> {
        digits_vec.sort_unstable();
        let num_digits = digits_vec.len();
        digits_vec.resize(N, 0);
        let digits_array: [usize; N] = digits_vec.try_into().expect("wtf");
        GameState {
            num_digits,
            digits: digits_array,
        }
    }

    fn as_vec(&self) -> Vec<usize> {
        let mut vec_value = self.digits.to_vec();
        vec_value.truncate(self.num_digits);
        vec_value
    }
}

impl<const N: usize> GameState<N> {
    pub(crate) fn next_states(&self) -> HashMap<GameState<N>, Vec<Operation>> {
        let mut possible_states: HashMap<GameState<N>, Vec<Operation>> = HashMap::new();
        for i1 in 0..self.num_digits {
            for i2 in 0..self.num_digits {
                if i2 == i1 {
                    continue;
                };
                for operation in [Add, Subtract, Multiply, Divide] {
                    let mut new_state = self.as_vec();
                    let num1 = new_state.remove(max(i1, i2));
                    let num2 = new_state.remove(min(i1, i2));
                    let result = operation.operate(num1, num2);

                    if let Some(result) = result {
                        let state = GameState::new([&new_state[..], &[result]].concat());
                        let operation = Operation {
                            op_type: operation,
                            num1,
                            num2,
                        };
                        if let Some(operations) = possible_states.get_mut(&state) {
                            operations.push(operation);
                        } else {
                            possible_states.insert(state, vec![operation]);
                        }
                    }
                }
            }
        }
        possible_states
    }
}
