//! Human player.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::positions::{BoardPosition, Position};
use std::fmt;
use std::io;
use std::io::BufRead;

/// Let a human enter moves on stdin.
pub struct Human();
impl fmt::Display for Human {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Human")
    }
}

fn ask_cell() -> Result<(u8, u8), io::Error> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line)?;
    let mut words = line.split_whitespace();
    let y: u8 = words
        .next()
        .unwrap()
        .parse()
        .map_err(|_| io::ErrorKind::Other)?;
    let x: u8 = words
        .next()
        .unwrap()
        .parse()
        .map_err(|_| io::ErrorKind::Other)?;
    Ok((x, y))
}

fn ask_move() -> Result<Movement, io::Error> {
    println!("Enter start point");
    let (sx, sy) = ask_cell()?;
    let start_position = Position::from_2d(sx, sy);
    println!("Enter end point");
    let (ex, ey) = ask_cell()?;
    let end_position = Position::from_2d(ex, ey);
    //Check that the distance between the 2 points is either 1 or 2
    match start_position.distance_to(end_position) {
        1 => Ok(Movement::Duplicate(end_position)),
        2 => Ok(Movement::Jump(start_position, end_position)),
        _ => {
            println!("Invalid movement. Please enter a valid move.");
            ask_move()
        }
    }
}

impl Strategy for Human {
    fn compute_next_move(&mut self, configuration: &Configuration) -> Option<Movement> {
        if configuration.movements().next().is_some() {
            loop {
                //Check distance OK
                if let Ok(movement) = ask_move() {
                    //Check position to travel to is valid in board
                    if configuration.check_move(&movement) {
                        return Some(movement);
                    } else {
                        println!("invalid movement (are you playing your color ?)");
                    }
                }
            }
        } else {
            None
        }
    }
}
