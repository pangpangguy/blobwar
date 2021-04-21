//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;

/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

//Minmax algorithm
//rec_depth  = recursion depth
fn minimax(state: &Configuration, rec_depth: u8) -> (i8, Option<Movement>) {
    if rec_depth == 0 || state.movements().next().is_none() {
        return (state.value(), None);
    }

    //Pour chaque move, on calcul recursivement la meilleur score de ses enfants (jusqu'on arrive aux feuilles)
    let mut best_score = 65;
    let mut best_mv = None;
    for m in state.movements() {
        let (mut mv_score, _) = minimax(&state.play(&m), rec_depth - 1);
        mv_score *= -1;
        if mv_score < best_score {
            //-ve car le tour prochain, la valeur calculé est celui de l'opponent
            best_score = mv_score;
            best_mv = Some(m);
        }
    }
    //  println!("{:?}", (best_score, overall_best_mv));
    (best_score, best_mv)
}

fn minimaxv2(state: &Configuration, rec_depth: u8, is_minimizer: bool) -> (i8, Option<Movement>) {
    //Reached leaf node
    if rec_depth == 0 || state.movements().next().is_none() {
        return (state.value(), None);
    }

    if is_minimizer {
        let mut best_score = 65;
        let mut best_move = None;

        //    println!("start");
        for mv in state.movements() {
            let (mut score, _) = minimaxv2(&state.play(&mv), rec_depth - 1, false);
            if score < best_score {
                best_score = score;
                best_move = Some(mv);
            }
            //      println!("Score :{:?}", score);
        }
        //    println!("best :{:?}", best_score);
        (best_score, best_move)
    } else {
        //Maximiser.
        let mut best_score = -65;
        let mut best_move = None;
        for mv in state.movements() {
            let (mut score, _) = minimaxv2(&state.play(&mv), rec_depth - 1, true);
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
        }
        (best_score, best_move)
    }
}

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let (valeur, mv) = minimaxv2(state, self.0, true);
        // println!("Best val = {:?}, best mv = {:?}", valeur, mv);
        mv

        //     let (valeur, mv) = minimax(state, self.0);
        //   mv
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

/// Anytime min max algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn min_max_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        movement.store(MinMax(depth).compute_next_move(state));
    }
}
