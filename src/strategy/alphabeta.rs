//! Alpha - Beta algorithm.
use std::collections::HashMap;
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

fn alphabeta_memo(
    state: &Configuration,
    rec_depth: u8,
    mut alpha: i8,
    mut beta: i8,
    is_minimizer: bool,
    seen_states: &mut HashMap<String, (i8, Option<Movement>)>,
) -> (i8, Option<Movement>) {
    if rec_depth == 0 || state.movements().next().is_none() {
        return (state.value(), None);
    }
    //Vérifie si le state a été déjà exploré
    let game_state_key = state.serialize();
    if seen_states.contains_key(&game_state_key) {
        let (a, b) = seen_states.get(&game_state_key).unwrap();
        return (*a, *b);
    }
    //Minimiser
    if is_minimizer {
        let mut best_score = 65;
        let mut best_move = None;
        for mv in state.movements() {
            let (mut score, _) = alphabeta_memo(
                &state.play(&mv),
                rec_depth - 1,
                alpha,
                beta,
                false,
                seen_states,
            );
            score *= -1; //Inverser le signe pour obtenir notre score. On cherche à minimser ce score
                         //Store the score
            if score < best_score {
                best_score = score;
                best_move = Some(mv);
            }
            if score < beta {
                beta = score;
            }
            if beta <= alpha {
                break;
            }
        }
        seen_states.insert(game_state_key, (best_score, best_move));
        (best_score, best_move)
    } else {
        //Maximiser (l'adversaire)
        let mut best_score = -65;
        let mut best_move = None;
        for mv in state.movements() {
            let (score, _) = alphabeta_memo(
                &state.play(&mv),
                rec_depth - 1,
                alpha,
                beta,
                true,
                seen_states,
            );
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
        seen_states.insert(game_state_key, (-best_score, best_move));
        (-best_score, best_move) //On renvoiee au minimiser le score de l'adversaire (donc inversion de signe)
    }
}

//On ordonne les choix selon leur valeurs. On commence par le noeud avec la meilleure valeur
//puisque c'est celui le plus probable d'être le noeud optimal.
fn alphabeta_memo_sort(
    state: &Configuration,
    rec_depth: u8,
    mut alpha: i8,
    mut beta: i8,
    is_minimizer: bool,
    seen_states: &mut HashMap<String, (i8, Option<Movement>)>,
) -> (i8, Option<Movement>) {
    if rec_depth == 0 || state.movements().next().is_none() {
        return (state.value(), None);
    }
    //Vérifie si le state a été déjà exploré
    let game_state_key = state.serialize();
    if seen_states.contains_key(&game_state_key) {
        let (a, b) = seen_states.get(&game_state_key).unwrap();
        return (*a, *b);
    }
    let mut sorted_moves: Vec<_> = state.movements().collect();
    //Minimiser
    if is_minimizer {
        let mut best_score = 65;
        let mut best_move = None;
        sorted_moves.sort_by_key(|mv| -state.play(&mv).value());
        for mv in sorted_moves {
            let (mut score, _) = alphabeta_memo_sort(
                &state.play(&mv),
                rec_depth - 1,
                alpha,
                beta,
                false,
                seen_states,
            );
            score *= -1; //Inverser le signe pour obtenir notre score. On cherche à minimser ce score
                         //Store the score
            if score < best_score {
                best_score = score;
                best_move = Some(mv);
            }
            if score < beta {
                beta = score;
            }
            if beta <= alpha {
                break;
            }
        }
        seen_states.insert(game_state_key, (best_score, best_move));
        (best_score, best_move)
    } else {
        //Maximiser (l'adversaire)
        sorted_moves.sort_by_key(|mv| state.play(&mv).value());
        let mut best_score = -65;
        let mut best_move = None;
        for i in (0..sorted_moves.len()).rev() {
            let (score, _) = alphabeta_memo_sort(
                &state.play(&sorted_moves[i]),
                rec_depth - 1,
                alpha,
                beta,
                true,
                seen_states,
            );
            if score > best_score {
                best_score = score;
                best_move = Some(sorted_moves[i]);
            }

            if score > alpha {
                alpha = score;
            }
            if beta <= alpha {
                break;
            }
        }
        seen_states.insert(game_state_key, (-best_score, best_move));
        (-best_score, best_move) //On renvoiee au minimiser le score de l'adversaire (donc inversion de signe)
    }
}
impl Strategy for AlphaBeta {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        //Alpha Beta de base
        let (score, mv) = alphabeta(state, self.0, -65, 65, true);
        
        //Alpha Beta avec memoization
        //let mut seen_states = HashMap::new();
        //let (score, mv) = alphabeta_memo(state, self.0, -65, 65, true, &mut seen_states);

        //Alpha Beta avec memozation et tirage
        //let (score, mv) = alphabeta_memo_sort(state, self.0, -65, 65, true, &mut seen_states);
        mv
    }
}
