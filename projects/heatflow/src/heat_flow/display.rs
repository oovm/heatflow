use std::fmt::{Display, Write};
use super::*;

impl<'i> Debug for LineView<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        for (i, x) in self.line.iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?;
            }
            write!(f, "{:.2}", x)?;
        }
        f.write_char(']')?;
        Ok(())
    }
}


impl Debug for HeatMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<LineView> = self.data.axis_iter(Axis(0)).map(|line| LineView { line }).collect();
        f.debug_struct("HeatMap")
            .field("x", &Range {
                start: self.area.anchor.x,
                end: self.area.anchor.x + self.area.side.0,
            })
            .field("y", &Range {
                start: self.area.anchor.y,
                end: self.area.anchor.y + self.area.side.1,
            })
            .field("size", &self.data.shape()[0])
            .field("h", &self.data.shape()[0])
            .field("data", &lines)
            .finish()
    }
}

impl Display for HeatMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<LineView> = self.data.axis_iter(Axis(0)).map(|line| LineView { line }).collect();
        f.debug_list().entries(lines).finish()
    }
}