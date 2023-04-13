mod errors;


pub use errors::{Error, Result};

mod queue;
mod heatflow;

pub use crate::queue::{Circular, iters::{GetCircular, MutCircular}};
pub use crate::heatflow::{HeatFlow, iters::{HeatFlowViewZ}};