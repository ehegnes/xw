use libc::{c_short, malloc};
use x11::xlib;

pub trait Drawable {
    fn draw(self, display: *mut xlib::Display, drawable: xlib::Drawable, gc: xlib::GC);
}

#[derive(Default, Clone)]
#[repr(C)]
pub struct Point {
    x: c_short,
    y: c_short,
}

impl From<(c_short, c_short)> for Point {
    fn from(t: (c_short, c_short)) -> Self {
        Point { x: t.0, y: t.1 }
    }
}

impl From<xlib::XPoint> for Point {
    fn from(xpoint: xlib::XPoint) -> Self {
        Point {
            x: xpoint.x,
            y: xpoint.y,
        }
    }
}

impl Drawable for Point {
    fn draw(self, display: *mut xlib::Display, drawable: xlib::Drawable, gc: xlib::GC) {
        unsafe {
            xlib::XDrawPoint(display, drawable, gc, self.x as i32, self.y as i32);
        }
    }
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct Segment {
    x1: c_short,
    y1: c_short,
    x2: c_short,
    y2: c_short,
}

impl From<xlib::XSegment> for Segment {
    fn from(xsegment: xlib::XSegment) -> Self {
        Segment {
            x1: xsegment.x1,
            y1: xsegment.y1,
            x2: xsegment.x2,
            y2: xsegment.y2,
        }
    }
}

impl Into<xlib::XSegment> for Segment {
    fn into(self) -> xlib::XSegment {
        xlib::XSegment {
            x1: self.x1,
            y1: self.y1,
            x2: self.x2,
            y2: self.y2,
        }
    }
}

impl From<(c_short, c_short, c_short, c_short)> for Segment {
    fn from(t: (c_short, c_short, c_short, c_short)) -> Self {
        Segment {
            x1: t.0,
            y1: t.1,
            x2: t.2,
            y2: t.3,
        }
    }
}

impl From<(Point, Point)> for Segment {
    fn from(t: (Point, Point)) -> Self {
        Segment {
            x1: t.0.x,
            y1: t.0.y,
            x2: t.1.x,
            y2: t.1.y,
        }
    }
}

/// For the madlads out there
impl From<((c_short, c_short), (c_short, c_short))> for Segment {
    fn from(t: ((c_short, c_short), (c_short, c_short))) -> Self {
        Segment {
            x1: (t.0).0,
            y1: (t.0).1,
            x2: (t.1).0,
            y2: (t.1).1,
        }
    }
}

impl Drawable for Segment {
    fn draw(self, display: *mut xlib::Display, drawable: xlib::Drawable, gc: xlib::GC) {
        unsafe {
            xlib::XDrawSegments(display, drawable, gc, vec![self.into()].as_mut_ptr(), 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::window::Window;
    use crate::XBuilder;

    //#[test]
    fn point() {
        XBuilder::new()
            .with_window(Window::new().name("Drawable Point").width(100).height(100))
            .add_color("magenta")
            .set_foreground("magenta")
            .draw(Point::from((50, 50)))
            .flush();
    }

    #[test]
    fn segment() {
        XBuilder::new()
            .with_window(
                Window::new()
                    .name("Drawable Segment")
                    .x(100)
                    .y(100)
                    .width(100)
                    .height(100),
            )
            .add_color("purple")
            .set_foreground("purple")
            .draw(Segment::from((0, 0, 100, 100)))
            .flush();
    }
}
