//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;

/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

/**Minmax algorithm**/
//Il y a deux players: Maximiser et Minimiser
//On itere sur chaque move, et on calcule recursivement la valeur minimale
//Chaque tour on alterne entre minimiser qui le meilleur score (le minimum) et le
//maximiser(l'adversaire) qui cherche à maximiser le score (Choisir le pire)
fn minimax(state: &Configuration, rec_depth: u8, is_minimizer: bool) -> (i8, Option<Movement>) {
    //Reached leaf node
    if rec_depth == 0 || state.movements().next().is_none() {
        return (state.value(), None);
    }

    //Minimiser
    if is_minimizer {
        let mut best_score = 65;
        let mut best_move = None;
        for mv in state.movements() {
            let (mut score, _) = minimax(&state.play(&mv), rec_depth - 1, false);
            score *= -1; //Inverser le signe pour obtenir notre score. On cherche à minimser ce score
            if score < best_score {
                best_score = score;
                best_move = Some(mv);
            }
        }
        (best_score, best_move)
    } else {
        //Maximiser (l'adversaire)
        //Le best_score ici est le notre (donc pas d'inversion de signe)
        //Le maximiser(l'adversaire) cherche à maximiser ce score
        let mut best_score = -65;
        let mut best_move = None;
        for mv in state.movements() {
            let (mut score, _) = minimax(&state.play(&mv), rec_depth - 1, true);
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
        }
        (-best_score, best_move) //On renvoiee au minimiser le score de l'adversaire (donc inversion de signe)
    }
}

/*Minimax Version 2
Version plus compacte, mais le code est moins inituive.
On differencie pas maximiser/minimiser, à chaque niveau de recursion on cherche toujours à minimiser*/
fn minimaxV2(state: &Configuration, rec_depth: u8) -> (i8, Option<Movement>) {
    if rec_depth == 0 || state.movements().next().is_none() {
        return (state.value(), None);
    }

    let mut best_score = 65;
    let mut best_mv = None;
    for m in state.movements() {
        let (mut mv_score, _) = minimaxV2(&state.play(&m), rec_depth - 1);
        mv_score *= -1;
        if mv_score < best_score {
            best_score = mv_score;
            best_mv = Some(m);
        }
    }
    (best_score, best_mv)
}

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let (_, mv) = minimax(state, self.0, true);
        //let (_, mv) = minimaxV2(state, self.0);
        mv
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
