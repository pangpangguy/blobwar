//! Alpha - Beta algorithm.
use std::fmt;

use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;

/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        let chosen_movement = AlphaBeta(depth).compute_next_move(state);
        movement.store(chosen_movement);
    }
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBeta(pub u8);

impl fmt::Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}

fn alphabeta(state: &Configuration, rec_depth: u8, alpha: i8, beta: i8) -> (i8, Option<Movement>) {
    if rec_depth == 0 || state.movements().next().is_none() {
        return (state.value(), None);
    }
    let mut best_score = 65;
    let mut best_mv = None;
    for m in state.movements() {
        let (mv_score, mv) = alphabeta(&state.play(&m), rec_depth - 1, alpha, beta);
        if -mv_score < best_score {
            best_score = -mv_score;
            best_mv = mv;

            //On vérifie si cette nouvelle best_score du noeud est meilleur (inf à beta)

            //On vérifie si
        }
    }
    //  println!("{:?}", (best_score, overall_best_mv));
    (best_score, best_mv)
}
impl Strategy for AlphaBeta {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let (score, mv) = alphabeta(state, self.0, 65, 65);
        mv
    }
}
