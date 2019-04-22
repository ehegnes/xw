//! .draw(vec![Segment(), Segment()])
//!

use libc::c_short;
use x11::xlib;

trait Drawable {
    fn draw();
}

#[derive(Default)]
struct Point {
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

impl Point {
    pub fn new(x: c_short, y: c_short) -> Self {
        Point { x, y }
    }
}

struct Segment {
    x1: c_short,
    y1: c_short,
    x2: c_short,
    y2: c_short,
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
