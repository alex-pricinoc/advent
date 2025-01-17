use std::cell::RefCell;
use std::sync::Mutex;
use std::time::Instant;

pub use itertools;

pub mod input;

pub mod prelude {
    pub use std::collections::HashMap;

    pub use itertools::{iproduct, Itertools};
}

static INPUT: Mutex<Option<&'static str>> = Mutex::new(None);

struct Timer(Instant);

thread_local!(static TIMER: RefCell<Option<Timer>> = RefCell::new(None));

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
