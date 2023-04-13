mod errors;


pub use errors::{Error, Result};

mod queue;

pub use self::queue::{Circular, iters::{GetCircular, MutCircular}};