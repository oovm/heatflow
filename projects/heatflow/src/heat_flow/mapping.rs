use super::*;


impl HeatMap {
    pub fn get_w(&self) -> usize {
        self.data.shape()[0]
    }
    pub fn get_h(&self) -> usize {
        self.data.shape()[1]
    }
    pub fn get_size(&self) -> (usize, usize) {
        (self.get_w(), self.get_h())
    }
    pub fn get_time(&self) -> usize {
        self.time
    }
    pub fn get_min_max(&self) -> Range<f32> {
        self.range.clone()
    }
}
