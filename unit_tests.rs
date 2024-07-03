// Unit Tests




pub fn snuggle(bunnies: u128) -> u128 {
    bunnies * 8 // mr incredible uncanny
}


#[cfg(test)] // Only run while testing
mod test {
    use super::*; // One of three situations where it is valid to use a wildcard to use imports

    #[test]
    fn snuggling_bunnies_multiply() {
        assert_eq!(snuggle(2),16);
    }
}

// "cargo test" runs your tests
// Testing is a first-class part of the Rust language
// Tells you the results: What passed, what failed, what was ignored, what was meaured, filtered
// out, and compile time
//

#[should_panic] // intended
#[test]
fn scared_bunny() {
    panic!("Hop away.");
}

// "Crate" in rust has two definitions

// 1. crate = package (0/1 library, any number of binaries)
// 2. crate = a library or a binary (Used by Rust-lang or Cargo tooling)

// Doc tests
// You can use "Doc-tests" not for binaries, but for libraries

// Here is a documentation test for the snuggle() method

/// # Example
///
/// ```
/// # use hello::snuggle;
/// let bunnies = snuggle(5);
/// assert_eq!(bunnies,40);
/// ```
pub fn snuggle(bunnies: u128) -> u128 {
    bunnies * 8
}

// Appears as:

Example
-------
let bunnies = snuggle(5)
assert_eq!(bunnies,40);

// When we run Doc-tests hello we get
// This gives us the file and the line of the tested function
test src/lib.rs - snuggle (line 3) ... ok


// A test can optionally return a result

#[test]
fn bunny_result() -> Result<(), ParseIntError> {
    let num_bunnies: u64 = "four".parse()?;
    assert_eq!(num_bunnies, 4);
    Ok(())
}

// A test failure gives a stdout prompt in the terminal
// It tells us the type of error and its respective kind if this is enabled
// There are no doc-tests here, because rust tests are primed to fail by default
// In this way they are reminiscent of the Spanish Inquisition Court, being deemed failed until
// proven successful. 
// Unlike the Spanish Court, a very high failure rate is actually conducive to writing good code




































