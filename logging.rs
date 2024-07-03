// Logging 
// Logging is not handled by the standard library
// It is handled by the log library

[package]
name = "puzzles"
version = "0.1.0"
edition: "2021"

[dependencies]
thiserror = "1.0"
puzzles = { path = "../puzzles" } // Now you don't need to use println!, it logs directly
log = "0.4"
env_logger = "0.9"

// Order of priority for logs:

use log::{error,warn,info,debug,trace}

error!(" ");
warn!(" ");
info!(" ");
debug!("More info");
trace!("Everything");


// Macros encode the log level and require a message argument but can also include its own syntax

warn!(target: "puzzle", "Pay attention");




// An example

puzzles/src/lib.rs

use log::{error, info};

impl Puzzle {
    /// Make a new puzzle!
    pub fn new() -> Self {
        let puzzle = Default::default();
        info!("Created a puzzle with new(): {:?}", puzzle);
        puzzle
    }
    /// Load a puzzle from a new file
    pub fn from_file(_fh: File) -> Result<Self, PuzzleError> {
        error!("This file is missing a piece!");
        Err(PuzzleError::MissingPiece)
    }
}

puzzles/src/main.rs

use anyhow::{Context,Result};

...

fn main() -> Result<()> {
    env_logger::init();
    let puzzle = get_puzzle("puzzle.dat").context("Couldn't get the first puzzle") {
        Ok(p) => p,
        Err(_) => Puzzle::new(),
    }
    info!("Playing puzzle: {}", puzzle.name);
    Ok(())
}

// Little bit more work,
// The log library defines a simple interface for all loggers, allowing global compabitility

// The log library is just a set of pipes, you need to decide where the information goes
// We'll use env_logger because it's very simple, but this is where you'd hook up stuff like
// log4s, fern, a cloud provider, Splunk/Datadog, the data engineers, custom applications? The
// possibilities are endless

// We will use env_logger for this purpose


