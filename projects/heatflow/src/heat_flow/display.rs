use std::fmt::{Display, Write};
use super::*;

struct Times<'i> {
    terms: &'i [usize],
}

impl<'i> Debug for Times<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, x) in self.terms.iter().enumerate() {
            if i != 0 {
                f.write_str(" × ")?;
            }
            write!(f, "{}", x)?;
        }
        Ok(())
    }
}

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
        let lines: Vec<LineView> = self.data.axis_iter(Axis(1)).map(|line| LineView { line }).collect();
        let (w, h) = self.data.dim();
        let size = Times { terms: &[w, h] };
        f.debug_struct("HeatMap")
            .field("size", &size)
            .field("x", &Range {
                start: self.area.anchor.x,
                end: self.area.anchor.x + self.area.side.0,
            })
            .field("y", &Range {
                start: self.area.anchor.y,
                end: self.area.anchor.y + self.area.side.1,
            })
            .field("time", &self.time)
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