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

//Beta - Le meilleur (minimum) score garanti pour le minimisesr (Nous)
//Alpha - Le meilleur (maximum) score garanti pour le maximiser (L'adversaire)
fn alphabeta(
    state: &Configuration,
    rec_depth: u8,
    mut alpha: i8,
    mut beta: i8,
    is_minimizer: bool,
) -> (i8, Option<Movement>) {
    if rec_depth == 0 || state.movements().next().is_none() {
        return (state.value(), None);
    }

    //Minimiser
    if is_minimizer {
        let mut best_score = 65;
        let mut best_move = None;
        for mv in state.movements() {
            let (mut score, _) = alphabeta(&state.play(&mv), rec_depth - 1, alpha, beta, false);
            score *= -1; //Inverser le signe pour obtenir notre score. On cherche à minimser ce score
            if score < best_score {
                best_score = score;
                best_move = Some(mv);
            }

            //Mise à jour beta
            if score < beta {
                beta = score;
            }

            //Si le maximiser est garanti d'une valeur (alpha) meilleure (plus) que ce qu'on (le minimiser)
            //peut avoir dans ce noeud, le maximiser va choisir le noeud de alpha donc c'est inutile de
            //explorer les noeud fils restants
            if beta <= alpha {
                break;
            }
        }
        (best_score, best_move)
    } else {
        //Maximiser (l'adversaire)
        let mut best_score = -65;
        let mut best_move = None;
        for mv in state.movements() {
            let (score, _) = alphabeta(&state.play(&mv), rec_depth - 1, alpha, beta, true);
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }

            if score > alpha {
                alpha = score;
            }
            if beta <= alpha {
                break;
            }
        }

        (-best_score, best_move) //On renvoiee au minimiser le score de l'adversaire (donc inversion de signe)
    }
}
impl Strategy for AlphaBeta {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let (score, mv) = alphabeta(state, self.0, -65, 65, true);
        mv
    }
}
