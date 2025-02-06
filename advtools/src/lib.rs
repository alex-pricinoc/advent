use std::cell::RefCell;
use std::fmt::Display;
use std::sync::Mutex;
use std::time::Instant;

pub use {itertools, num, rayon};

pub mod grid;
pub mod input;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use std::{iter, mem};

    pub use hashbrown::{HashMap, HashSet};
    pub use itertools::{iproduct, Itertools};
    pub use memoize::memoize;
    pub use regex::{Captures, Regex};

    pub use super::Solution;
}

static INPUT: Mutex<Option<&'static str>> = Mutex::new(None);

struct Timer(Instant);

thread_local!(static TIMER: RefCell<Option<Timer>> = const { RefCell::new(None) });

impl Timer {
    fn start() {
        TIMER.with(|k| *k.borrow_mut() = Some(Timer(Instant::now())));
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("   Elapsed: {:?}", self.0.elapsed());
    }
}

#[derive(PartialEq, Debug)]
pub struct Solution<A: Display, B: Display>(pub A, pub B);

impl<A: Display, B: Display> Display for Solution<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "part 1: {}\npart 2: {}", self.0, self.1)
    }
}

#[must_use]
pub fn digits(n: usize) -> u32 {
    n.checked_ilog10().map_or(1, |d| d + 1)
}
