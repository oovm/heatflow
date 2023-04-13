use super::*;


impl HeatMap {
    pub fn lines(&self) -> impl Iterator<Item=ArrayView1<f32>> {
        self.data.axis_iter(Axis(0))
    }
}

impl HeatFlow {
    pub fn new(area: Rectangle<f32>, resolution: f32, z_index: usize) -> Self {
        // densely packed on the z-axis, contributes to better performance
        let w = (area.side.0 / resolution).ceil() as usize;
        let h = (area.side.1 / resolution).ceil() as usize;
        let data = Array3::zeros((w, h, z_index));
        Self {
            area,
            data,
            time: 0,
        }
    }
    pub fn get_area(&self) -> Rectangle<f32> {
        self.area
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
    #[inline]
    fn get_z_index(&self) -> usize {
        self.time % self.get_z()
    }
    pub fn get_time(&self) -> usize {
        self.time
    }


    pub fn sampling(&mut self, point: Point<f32>, weight: f32) {
        let x = ((point.x - self.area.anchor.x) / self.area.side.0 * self.get_w() as f32).floor() as usize;
        let y = ((point.y - self.area.anchor.y) / self.area.side.1 * self.get_h() as f32).floor() as usize;
        let z = self.get_z_index();
        self.data[[x, y, z]] += weight;
    }
    pub fn time_fly(&mut self) {
        // avoid overflow when time reaches usize::MAX
        self.time = self.time.saturating_add(1);
        // make z-index = self.current all zero
        self.data.slice_mut(s![.., .., self.get_z_index()]).fill(0.0);
    }


    // add value over z-index
    pub fn as_heatmap(&self) -> HeatMap {
        let mut result = Array2::zeros((self.get_w(), self.get_h()));
        for (p, zs) in self.view_z() {
            match result.get_mut([p.x, p.y]) {
                Some(s) => {
                    *s += zs.sum()
                }
                None => {}
            }
        }
        let min = result.iter().fold(f32::MAX, |a, b| a.min(*b));
        let max = result.iter().fold(f32::MIN, |a, b| a.max(*b));
        HeatMap {
            area: self.area,
            data: result,
            time: self.time,
            range: min..max,
        }
    }
}
