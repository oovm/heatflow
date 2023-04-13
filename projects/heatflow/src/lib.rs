mod errors;


pub use errors::{Error, Result};

mod queue;

pub use crate::queue::{Circular, iters::{GetCircular, MutCircular}};