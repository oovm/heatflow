use ndarray::{ArrayView2, Ix3};
use ndarray::iter::IndexedIter;
use shape_core::Point3D;
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

pub struct HeatFlowViewXYZ<'i> {
    tensor: IndexedIter<'i, f32, Ix3>,
}


impl<'i> Iterator for HeatFlowViewZ<'i> {
    type Item = (Point<usize>, ArrayView1<'i, f32>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y >= self.tensor.get_h() {
            return None;
        }
        let result = self.tensor.slice_z(self.current_x, self.current_y);
        self.current_x += 1;
        if self.current_x >= self.tensor.get_w() {
            self.current_x = 0;
            self.current_y += 1;
        }
        let point = Point::new(self.current_x, self.current_y);
        Some((point, result))
    }
}

impl<'i> Iterator for HeatFlowViewXY<'i> {
    type Item = (usize, ArrayView2<'i, f32>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_z >= self.tensor.get_h() {
            return None;
        }
        let result = self.tensor.slice_xy(self.current_z);
        self.current_z += 1;
        Some((self.current_z, result))
    }
}

impl<'i> Iterator for HeatFlowViewXYZ<'i> {
    type Item = (Point3D<usize>, f32);

    fn next(&mut self) -> Option<Self::Item> {
        let ((x, y, z), v) = self.tensor.next()?;
        let point = Point3D {
            x,
            y,
            z,
        };
        Some((point, *v))
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
    pub fn view_xyz(&self) -> HeatFlowViewXYZ {
        HeatFlowViewXYZ {
            tensor: self.data.indexed_iter()
        }
    }
    // eliminate data life cycle
    #[inline(always)]
    fn slice_z(&self, x: usize, y: usize) -> ArrayView1<f32> {
        self.data.slice(s![x, y, ..])
    }
    // eliminate data life cycle
    #[inline(always)]
    fn slice_xy(&self, z: usize) -> ArrayView2<f32> {
        self.data.slice(s![.., .., z])
    }
}