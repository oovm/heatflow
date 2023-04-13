use ndarray::ArrayView2;
use super::*;

pub struct HeatFlowViewZ<'i> {
    tensor: &'i HeatFlow,
    current_x: usize,
    current_y: usize,
}

pub struct HeatFlowViewXY<'i> {
    tensor: &'i HeatFlow,
    current_z: usize,
}


impl<'i> Iterator for HeatFlowViewZ<'i> {
    type Item = (usize, usize, ArrayView1<'i, f32>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y > self.tensor.get_h() {
            return None;
        }
        let result = self.tensor.slice_z(self.current_x, self.current_y);
        self.current_x += 1;
        if self.current_x > self.tensor.get_w() {
            self.current_x = 0;
            self.current_y += 1;
        }
        Some((self.current_x, self.current_y, result))
    }
}

impl<'i> Iterator for HeatFlowViewXY<'i> {
    type Item = (usize, ArrayView2<'i, f32>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_z > self.tensor.get_h() {
            return None;
        }
        let result = self.tensor.slice_xy(self.current_z);
        self.current_z += 1;
        Some((self.current_z, result))
    }
}

impl HeatFlow {
    /// View values on z-index
    pub fn view_z(&self) -> HeatFlowViewZ {
        HeatFlowViewZ {
            tensor: self,
            current_x: 0,
            current_y: 0,
        }
    }
    pub fn view_xy(&self) -> HeatFlowViewXY {
        HeatFlowViewXY {
            tensor: self,
            current_z: 0,
        }
    }
    // Eliminate life cycle
    #[inline(always)]
    fn slice_z(&self, x: usize, y: usize) -> ArrayView1<f32> {
        self.data.slice(s![x, y, ..])
    }
    // Eliminate life cycle
    #[inline(always)]
    fn slice_xy(&self, z: usize) -> ArrayView2<f32> {
        self.data.slice(s![.., .., z])
    }
}