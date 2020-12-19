//! Simulate dice rolls and coin flips. For now, this relies on the default
//! thread-local RNG provided by the `rand` crate.

use rand::distributions::uniform::{SampleUniform, Uniform};
use rand::prelude::*;
// use rand::distributions::uniform::SampleUniform;

/// Trait indicating a simulation of rolling some kind of die.
///
/// Since dice may be marked with numbers, letters, or arbitrary
/// symbols, the trait is generic on the type returned from a roll.
pub trait RollableDie<T> {
    /// Simulate a roll of the die using a given random or pseudo-random
    /// number generator to determine the result.
    fn roll<R: Rng>(&self, rng: &mut R) -> T;
}

/// Simulate a die roll using a reasonably good default PRNG.
pub fn roll<T, D: RollableDie<T>>(d: &D) -> T {
    let mut rng = thread_rng();
    d.roll(&mut rng)
}

/// Simulate a number of rolls and return all of the results.
pub fn n_rolls<T, D>(n: usize, d: &D) -> Vec<T>
where
    D: RollableDie<T>,
{
    let mut rng = thread_rng();
    (0..n).map(|_| d.roll(&mut rng)).collect()
}

/// Create a n-sided die numbered 1..n inclusive. Uses a signed
/// integer so that the results of die rolls can be used directly
/// in calculations that may have negative results due to bonuses
/// or penalties.
pub fn n_sided(n: i32) -> Result<RangeDie<i32>, &'static str> {
    if n < 1 {
        Err("Invalid number of sides for a die.")
    } else {
        Ok(RangeDie::new(1, n))
    }
}

/// Convenience wrapper for 100-sided (percentile) dice.
pub fn d100() -> RangeDie<i32> {
    n_sided(100).unwrap()
}

/// Convenience wrapper for 20-sided dice.
pub fn d20() -> RangeDie<i32> {
    n_sided(20).unwrap()
}

/// Convenience wrapper for 12-sided dice.
pub fn d12() -> RangeDie<i32> {
    n_sided(12).unwrap()
}

/// Convenience wrapper for 10-sided dice.
pub fn d10() -> RangeDie<i32> {
    n_sided(10).unwrap()
}

/// Convenience wrapper for 8-sided dice.
pub fn d8() -> RangeDie<i32> {
    n_sided(8).unwrap()
}

/// Convenience wrapper for 6-sided dice.
pub fn d6() -> RangeDie<i32> {
    n_sided(6).unwrap()
}

/// Convenience wrapper for 4-sided dice.
pub fn d4() -> RangeDie<i32> {
    n_sided(4).unwrap()
}

/// Convenience wrapper for 2-sided dice (coin flip with numeric value).
pub fn d2() -> RangeDie<i32> {
    n_sided(3).unwrap()
}

/// Represent and simulate a die with arbitrary data or symbols on its
/// faces. There is no requirement for faces to be unique; this is
/// intentional, as it allows construction of "averaging" dice and other
/// dice with some identical faces in an intuitive manner.
#[derive(Clone, Debug)]
pub struct GenericDie<T: Clone> {
    faces: Vec<T>,
}

impl<T: Clone> RollableDie<T> for GenericDie<T> {
    fn roll<R: Rng>(&self, rng: &mut R) -> T {
        let i = rng.gen_range(0, self.faces.len());
        self.faces[i].clone()
    }
}

impl<T: Clone> RollableDie<T> for &GenericDie<T> {
    fn roll<R: Rng>(&self, rng: &mut R) -> T {
        let i = rng.gen_range(0, self.faces.len());
        self.faces[i].clone()
    }
}

impl<T: Clone> GenericDie<T> {
    /// Create a die from an iterator of values/items representing the possible
    /// results of a roll.
    ///
    /// ```
    /// use rusty_dice::*;
    ///
    /// let abc = GenericDie::new_from(['a', 'b', 'c'].into_iter());
    /// let first_result = roll(&abc);
    /// println!("{}", first_result);
    /// ```
    pub fn new_from<I: ExactSizeIterator<Item = T>>(iterator: I) -> Self {
        GenericDie {
            faces: iterator.into_iter().collect(),
        }
    }
}

/// Represents a die whose faces are numbered n..m inclusive.
pub struct RangeDie<T: SampleUniform> {
    faces: Uniform<T>,
}

impl<T: SampleUniform> RollableDie<T> for RangeDie<T> {
    fn roll<R: Rng>(&self, rng: &mut R) -> T {
        rng.sample(&self.faces)
    }
}

impl<T: SampleUniform> RollableDie<T> for &RangeDie<T> {
    fn roll<R: Rng>(&self, rng: &mut R) -> T {
        rng.sample(&self.faces)
    }
}

impl<T: SampleUniform> RangeDie<T> {
    /// Create a die with specified minimum and maximum values.
    pub fn new(min: T, max: T) -> Self {
        RangeDie {
            faces: Uniform::new_inclusive(min, max),
        }
    }
}

/// Represent coin flip results in a readable way.
#[derive(Eq, PartialEq)]
pub enum CoinFace {
    Heads,
    Tails,
}

/// Simulate coin flips.
pub fn coin_flip() -> CoinFace {
    match thread_rng().gen_range(0, 2) {
        0 => CoinFace::Heads,
        1 => CoinFace::Tails,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_1000_coins_without_crashing() {
        for _ in 0..1000 {
            let result = coin_flip();
            assert!(result == CoinFace::Heads || result == CoinFace::Tails);
        }
    }

    #[test]
    fn roll_many_d20s() {
        let rolls: Vec<i32> = n_rolls(100, &d20());
        assert_eq!(rolls.len(), 100);
        assert!(rolls.iter().sum::<i32>() >= 100);
    }
}
