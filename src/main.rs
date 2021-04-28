extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::Strategy;
use blobwar::strategy::{AlphaBeta, Greedy, Human, MinMax, RandomizedGreedy};
use std::time::Duration;

fn main() {
    //let board = Board::load("x").expect("failed loading board");
    let board = Default::default();
    let mut game = Configuration::new(&board);
    print!("Starting game...");
    game.battle(AlphaBeta(6), AlphaBeta(2));
    //test_algo(10);
}

//Function for testing purposes
//Prints outcome on terminal. +1 = Red wins, -1 = Blue wins, 0 = Draw
pub fn test_algo(nb_test: u64) {
    let mut results: Vec<i8> = Vec::new();
    for i in 0..nb_test {
        let board = Default::default();
        let mut game = Configuration::new(&board);
        //println!("Test {:?} / {}", i + 1, nb_test);
        let (outcome, compute_duration) = game.battle_no_display(AlphaBeta(4), RandomizedGreedy());
        //Result of the game
        results.push(outcome);

        println!("Time taken for this game : {:?}", compute_duration);
    }
    println!("Test Completed");
}
