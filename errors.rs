// Errors

// Jane Lusby - Error Handling Isn't All About Errors (RustConf 2020)
//


// Making Errors
// #1: errors are "enum"

pub enum PuzzleError {
    WontFit(u16),
    MissingPiece,
}

// #2 Group your errors as variants of as few enums as makes sense

// A great example is std::io::ErrorKind and std::num::IntErrorKind
// Here all of the POSSIBLE types of error are grouped into a non exhaustive list for the io
// library, so you can add more

#[non_exhaustive]
pub enum ErrorKind {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ...
    UnexpectedEof,
    OutOfMemory,
    Other,
}

#[non_exhaustive]
pub enum IntErrorKind {
    Empty,
    InvalidDigit,
    PosOverflow,
    NegOverflow,
    Zero,
}

// #3. Only return YOUR errors from the public library
// Convert errors that are separate to your library into errors that are relevant to your library

pub enum FractalError {
    FractalError,     ----->
}

pub enum PuzzleError {
    WontFit(u16),
    MissingPiece,
    PrettyImageless,  <-----
}

// 1. The external dependency can break your public API
// 2. It prevents you from chaging your own library's backend implementation without breaking your
//    own public API


// #4. Non-Exhaustive
// Your enums should be non-exhaustive

#[non_exhaustive]
pub enum PuzzleError {
    WontFit(u16),
    MissingPiece,
}

// With #[non_exhaustive], you can't do a match statement without a wildcard
// This prevents users' code breaking

match error {
    PuzzleError::WontFit(_) => {}
    PuzzleError::MissingPiece => {}
    _ => {}
}

// #5. Debug + Display traits should be implemented in your Error

std::Error

pub trait Error: Debug + Display


use std::fmt::{Display, Formatter};

impl Display for PuzzleError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        use PuzzleError::*;
        match self {
            MissingPiece => write!(f, "Missing a piece")
            WontFit(n) => write!(f, "Piece {} doesn't fit!", n),
        }
    }
 }

// Now implement error

use std::error:Error;

impl Error for PuzzleError {}

// Put the code together

// #5b. Use thiserror
// This is a rust crate (so add it to Cargo.toml)

[package]
name = "puzzles"
version = "0.1.0"
edition = "2021"

[dependencies]
thiserror = "1.0"

// ...

use thiserror:Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PuzzleError {
    #[error("Piece {0} doesn't fit!")]
    WontFit(u16),
    #[error("Missing a piece")]
    MissingPiece,
}

// Far simpler



// Handling Errors
#[must_use]
enum Result<T, E> {
    Ok(T)
    Err(E)
}

// The panic macro is the nuclear option
// unwrap is the same without the message
// If all else fails, use it 
// Never use it in a library

// Manual
panic!("DNNGG")

// if result is a Result:Err
result.expect("DNNGG");

// Without a message but does same thing
result.unwrap();

// Wouldn't rely on these for a library either

// Nothing wrong with returning errors as a handling mechanism either
// Hate those 2000000 line try-catch statements in java


fn poem() -> Result<String, io::Error> {
    let file = match File::Open("pretty_words.txt") {
        Ok(f) => f,                 // unwrap the value
        Err(e) => return Err(e),    // turn it into an Error
    };

    // Operate on file

}

// This type of error handling is so common, we use the "?" macro

fn poem() -> Result<String, io::Error> {
    let file = File::open("pretty_words.txt")?;
    // Operate on file
}



pub fn autobots_rollout() -> Result<Vehicle, TransformerError> {
    let optimus = Transformer::new();
    optimus.stand()?.transform()?.rollout()?.chase()? // Every method here is error handled
}

// The anyhow package, along with eyre/snafu, is a mainstream dependency choice for idiomatic error
// handling


use anyhow::{Context, Result};
use puzzles::Puzzle;
use std::fs::File;

fn get_puzzle(filename: &str) -> Result<Puzzle> {
    let fh = File::open(filename)?;
        .with_context(|| format!("Can't open {}", filename))?;
    let puzzle = Puzzle::from_file(fh).context("Couldn't convert data to puzzle")?;
    Ok(puzzle)
}
fn main() -> Result<()> {
    let puzzle = get_puzzle("puzzle.dat").context("Couldn't get the first puzzle")?;
    println!("Playing puzzle: {}", puzzle.name);
    Ok(())
}

// Context 1: Couldn't get the first puzzle
// Context 2: Couldn't open the puzzle file
// Context 3: No such file or dirty

// A fix is implemented that allows puzzle.dat to be provided

// Context 1: Couldn't get the first puzzle
// Context 2: Couldn't convert data into a puzzle
// Context 3: Missing a piece

// with the right access code this can give you the full backtrace for the nightly build of any
// rust crate

$ rustup override set nightly


















































































