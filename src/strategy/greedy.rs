//! Dumb greedy algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt; // 0.7.2

/// Dumb algorithm.
/// Amongst all possible movements return the one which yields the configuration with the best
/// immediate value.
pub struct Greedy();

///Randomized version of Greedy
pub struct RandomizedGreedy();

impl fmt::Display for Greedy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Greedy")
    }
}
impl fmt::Display for RandomizedGreedy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RandomizedGreedy")
    }
}
impl Strategy for Greedy {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        //Scenario 1: Skip this turn
        let state_after_skip = state.skip_play();

        //Get the value of config of this move (skipping).
        //skip_play is used again to inverse current_player attribute in configuration
        let mut best_value = state_after_skip.skip_play().value();

        //Scenario 2: Make a move this turn
        //Iterate through all possible movements, for each of them calculate their value.
        //Retain the move with the lowest value. (or if there are multiple, retain only the first lowest)
        //best_move starts with None (skip play)
        let mut best_move = None;
        for mv in state.movements() {
            let state_after_move = state.play(&mv);
            let value = state_after_move.skip_play().value();

            if value < best_value {
                best_value = value;
                best_move = Some(mv);
            }
        }

        /*
        let all_moves: Vec<_> = state.movements().collect();
        let all_moves_values: Vec<_> = state
            .movements()
            .map(|x| {
                let statez = state.play(&x);
                statez.skip_play().value()
            })
            .collect();
        println!("{:?}", all_moves);
        println!("{:?}", all_moves_values);
        */
        best_move
    }
}

//Greedy but randomly chooses a move when there are multiple best values
impl Strategy for RandomizedGreedy {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let state_after_skip = state.skip_play();
        let mut best_value = state_after_skip.skip_play().value();

        let mut best_moves: Vec<Option<Movement>> = Vec::new();
        best_moves.push(None); //None for the skip play

        for mv in state.movements() {
            let state_after_move = state.play(&mv);
            let value = state_after_move.skip_play().value();

            if value == best_value {
                best_moves.push(Some(mv));
            } else if value < best_value {
                best_value = value;
                best_moves.clear();
                best_moves.push(Some(mv));
            }
        }

        let mut rng = thread_rng();
        let random_best_move = best_moves.choose(&mut rng);
        *random_best_move.unwrap()
    }
}
