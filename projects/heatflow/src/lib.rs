mod errors;


pub use errors::{Error, Result};

mod queue;
mod heat_flow;

pub use crate::queue::{Circular, iters::{GetCircular, MutCircular}};
pub use crate::heat_flow::{HeatFlow, iters::{HeatFlowViewZ, HeatFlowViewXY}, HeatMap};
pub use shape_core::{Rectangle, Point};