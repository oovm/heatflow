use super::*;


impl HeatMap {
    pub fn get_w(&self) -> usize {
        self.data.shape()[0]
    }
    pub fn get_h(&self) -> usize {
        self.data.shape()[1]
    }
    pub fn get_size(&self) -> usize {
        self.data.shape()[0]
    }
    pub fn get_time(&self) -> usize {
        self.time
    }
}
