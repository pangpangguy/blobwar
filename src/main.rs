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
    // game.battle(MinMax(2), RandomizedGreedy());
    test_algo(1);
}

//Function for testing purposes
pub fn test_algo(nb_test: u64) {
    let mut results: Vec<i8> = Vec::new();
    let mut time_taken: Vec<Duration> = Vec::new();
    for i in 0..nb_test {
        let board = Default::default();
        let mut game = Configuration::new(&board);
        //println!("Test {:?} / {}", i + 1, nb_test);
        let (outcome, compute_duration) = game.battle_no_display(AlphaBeta(4), RandomizedGreedy());
        //Result of the game
        results.push(outcome);
        //Calculate average time taken to compute move (for first player)
        println!("Results : {:?}", compute_duration);
    }
    println!("Test Completed");
    println!("Results : {:?}", results);
    println!("Results : {:?}", time_taken);
}
