use std::fmt::{Debug, Formatter};
use std::ops::Range;
use ndarray::{Array2, Array3, ArrayView1, ArrayView2, Axis, s};
use shape_core::{Point, Rectangle};


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

pub struct HeatFlowViewZ<'a> {
    data: ArrayView2<'a, f32>,
    z_index: usize,
}

impl

impl Debug for HeatMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut lines = self.lines().map(|line| {
            line.iter().map(|x| format!("{:.2}", x)).collect::<Vec<_>>().join(", ")
        }).collect::<Vec<_>>();

        f.debug_struct("HeatMap")
            .field("x", &Range {
                start: self.area.anchor.x,
                end: self.area.anchor.x + self.area.side.0,
            })
            .field("y", &Range {
                start: self.area.anchor.y,
                end: self.area.anchor.y + self.area.side.1,
            })
            .field("w", &self.data.shape()[0])
            .field("h", &self.data.shape()[0])
            .field("data", &lines)
            .finish()
    }
}

impl HeatMap {
    pub fn lines(&self) -> impl Iterator<Item=ArrayView1<f32>> {
        self.data.axis_iter(Axis(0))
    }
}

impl HeatFlow {
    pub fn new(area: Rectangle<f32>, resolution: f32, z_index: usize) -> Self {
        // Arrays are arranged contiguously on the z-axis
        let w = (area.side.0 / resolution).ceil() as usize;
        let h = (area.side.1 / resolution).ceil() as usize;
        let data = Array3::zeros((w, h, z_index));
        Self {
            area,
            data,
            current: 0,
        }
    }
    pub fn get_w(&self) -> usize {
        self.data.shape()[0]
    }
    pub fn get_h(&self) -> usize {
        self.data.shape()[1]
    }
    pub fn get_z(&self) -> usize {
        self.data.shape()[2]
    }
    pub fn sampling(&mut self, point: Point<f32>, weight: f32) {
        let x = ((point.x - self.area.anchor.x) / self.area.side.0 * self.get_w() as f32).floor() as usize;
        let y = ((point.y - self.area.anchor.y) / self.area.side.1 * self.get_h() as f32).floor() as usize;
        if let Some(s) = self.data.get_mut([x, y, self.current]) {
            *s += weight;
        }
    }
    pub fn time_fly(&mut self) {
        self.current = (self.current + 1) % self.data.shape()[2];
        // make z-index = self.current all zero
        self.data.slice_mut(s![.., .., self.current]).fill(0.0);
    }

    /// View values on z-index
    pub fn view_zs(&self) -> ArrayView1<ArrayView1<f32>> {

    }

    // add value over z-index
    pub fn as_heatmap(&self) -> HeatMap {
        let mut result = Array2::zeros((self.get_w(), self.get_h()));
        for z in self.view_zs() {
            result += z.sum()
        }
        HeatMap {
            area: self.area,
            data: result,
            time: self.get_z(),
            range: 0.0..1.0,
        }
    }
}

#[test]
pub fn test() {
    let mut map = HeatFlow::new(Rectangle::new(Point::new(0.0, 0.0), (8.0, 6.0)), 1.0, 5);
    map.sampling(Point::new(0.5, 0.5), 1.0);
    map.time_fly();
    map.sampling(Point::new(0.5, 0.5), 1.0);
    println!("{:?}", map.data.shape());
    println!("{:#?}", map.as_heatmap());
}