use std::fmt::{Debug, Formatter};
use std::ops::Range;
use ndarray::{Array2, Array3, ArrayView1, Axis, s};
use shape_core::{Point, Rectangle};

pub mod iters;
mod display;
mod mapping;
mod flow;

/// A `w × h × z` tensor which stores the frequency on each grid.
pub struct HeatFlow {
    area: Rectangle<f32>,
    data: Array3<f32>,
    current: usize,
}

/// A `w × h` matrix which stores the sum of frequency on each grid.
pub struct HeatMap {
    area: Rectangle<f32>,
    data: Array2<f32>,
    time: usize,
    range: Range<f32>,
}

pub struct LineView<'i> {
    line: ArrayView1<'i, f32>,
}

#[test]
pub fn test() {
    let mut map = HeatFlow::new(Rectangle::new(Point::new(0.0, 0.0), (8.0, 6.0)), 1.0, 5);
    map.sampling(Point::new(0.5, 0.5), 1.0);
    map.sampling(Point::new(1.5, 0.5), 1.0);
    map.time_fly();
    map.sampling(Point::new(0.5, 0.5), 1.0);
    println!("{:?}", map.data.shape());
    println!("{:#?}", map.as_heatmap());
    println!("{:?}", map.data.iter().take(3).collect::<Vec<_>>());
}