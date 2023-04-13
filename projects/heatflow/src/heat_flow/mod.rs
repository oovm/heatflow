use std::fmt::{Debug, Formatter};
use std::ops::Range;
use ndarray::{Array2, Array3, ArrayView1, Axis, s};
use shape_core::{Point, Rectangle};

pub mod iters;
mod display;
mod mapping;
mod flow;

/// A `w × h × z` tensor which stores the frequency on each grid.
///
/// ## Complexity Analysis
///
/// Requires $O(whz)$ space to store the data.
///
/// Each time step takes $O(mn)$ time to erase the previous layer.
pub struct HeatFlow {
    area: Rectangle<f32>,
    data: Array3<f32>,
    time: usize,
}

/// A `w × h` matrix which stores the sum of frequency on each grid.
pub struct HeatMap {
    area: Rectangle<f32>,
    data: Array2<f32>,
    range: Range<f32>,
    time: usize,
}

pub struct LineView<'i> {
    line: ArrayView1<'i, f32>,
}

#[test]
pub fn test() {
    let mut map = HeatFlow::new(Rectangle::new(Point::new(0.0, 0.0), (8.0, 6.0)), 1.0, 5);
    map.sampling(Point::new(0.5, 0.5), 1.0);
    println!("(1, 1): {:?}", map.view_z().next().unwrap());
    map.time_fly();
    map.sampling(Point::new(1.5, 0.5), 2.0);
    println!("(1, 1): {:?}", map.view_z().next().unwrap());
    map.time_fly();
    map.sampling(Point::new(0.5, 0.5), 3.0);
    println!("(1, 1): {:?}", map.view_z().next().unwrap());

    println!("{:?}", map.data.shape());
    println!("{:#?}", map.as_heatmap());
    println!("{:?}", map.data.iter().take(6).collect::<Vec<_>>());
}