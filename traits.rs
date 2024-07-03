// Traits
//
// Structs, enums, closures and functions can implement traits

// A trait can be derived if it has this macro
#[derive(Debug)]

std::fmt::Debug

pub trait Debug {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>;
}

// As long as everything in your function is debug you can debug it

// place a curly bracket instead the template holder in your template string

println!("{:?}", puzzle)    // Debug
println!("{:#?}", puzzle)   // Pretty debug


#[derive(Debug)]
pub struct Puzzle {
    pub num_pieces: u32,
    pub name: String,
}

println!("{:#?}", puzzle);

// Output
Puzzle {
    num_pieces: 500,
    name: "Draconic Equestrian"
}


// Can't be directly used in code, the borrowed string subslice here has to be converted to a
// string



// Clone trait
// Allows a value to be cloned

std::clone::Clone

pub trait Clone: Sized {
    // Required method
    fn clone(&self) -> Self;

    // Provided method
    fn clone_from(&mut self, source: &Self) { ... }
}

#[derive(Clone, Debug)]
pub struct Puzzle {
    pub num_pieces: u32,
    pub name: String
}

let puzzle2 = puzzle.clone();

// Copy trait
// A subtrait of clone
// If you implement a trait you must also implement its parent traits

std::marker::Copy

pub trait Copy: Clone { }


// Implementing a trait
//
// 1. Bring trait into scope with a use statement
// 2. Boilerplate 
// 3. Implementation


// Default trait
// A trait for giving a type a useful default value.

pub trait Default: Sized {
    // Required method
    fn default() -> Self;
}

impl Default for Puzzle {
    fn default() -> Self {
        num_pieces: PUZZLE_PIECES,
        name: "Forest Lake".to_string();
    }
}

// Struct update syntax is quite common using Default trait

let puzzle = Puzzle {
    num_pieces: 3000,
    ..Default::default()
};


// PartialEq / Eq    (Equals)
// PartialEq tests for equality while Eq is a marker trait
//
// You can only use Eq if the equality logic is reflexive, transitive and symmetric
// e.g. if every possible value is equal to itself


// Floating point types have a quality called NaN (not a number) and implement PartialEq however do
// not implement Eq since
// NaN != NaN.

std::cmp::PartialEq

pub trait PartialEq<Rhs = Self>
    where 
        Rhs: ?Sized,
    {
        // Required method
        fn eq(&self, other: &Rhs) -> bool;

        // Provided method
        fn ne(&self, other: &Rhs) -> bool { ... }
    }

// The Ctrl C+V formatting here hurt my soul


// &self desugars to self: &Self, a pointer to the Self trait
// The Puzzle implementation can now be used as a key in a hashmap

impl PartialEq for Puzzle {
    fn eq(&self, other: &Self) -> bool {
        (self.num_pieces == other.num_pieces) && (self.name == other.name)
    }
}

impl Eq for Puzzle {}


// From / Into traits
// Value to value conversions while consuming the input value

std::convert::From
std::convert::Into

pub trait From<T>: Sized {
    // Required method
    fn from(value: T) -> Self;
}

pub trait Into<T>: Sized {
    // Required method
    fn into(self) -> T;
}

// the from/into functions run quickly but take up a serious amount of lines under the hood

From<Puzzle> for String
Into<String> for Puzzle

impl From<Puzzle> for String {
    fn from(puzzle: Puzzle) -> Self {
        puzzle.name
    }
}

let puzzle = Puzzle::default();
let s = String::from(puzzle);

pub fn show<T: Into<String>>(s: T) {
    println!("{}", s.into());
}

let puzzle = Puzzle::default();
show(puzzle);

// We can use an immutable reference to the puzzle if you want to clone a large struct

impl From<&Puzzle> for String {
    fn from(puzzle: &Puzzle) -> Self {
        puzzle.name.clone()
    }
}

// The show function doesn't have to be changed except for an immutable reference

pub fn show ...


show(&puzzle);
// "puzzle" is still available




























































