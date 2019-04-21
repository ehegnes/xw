use libc::c_ulong;
use x11::xlib::{Display, XBlackPixel, XDefaultScreen, XWhitePixel};

#[derive(Debug)]
pub struct WindowAttributes {
    pub border_pixel: c_ulong,
    pub background_pixel: c_ulong,
}

impl WindowAttributes {
    pub fn from_display(display: *mut Display) -> Self {
        WindowAttributes {
            border_pixel: unsafe { XBlackPixel(display, XDefaultScreen(display)) },
            background_pixel: unsafe { XWhitePixel(display, XDefaultScreen(display)) },
        }
    }

    pub fn border_pixel(mut self, p: c_ulong) -> Self {
        self.border_pixel = p;
        self
    }

    pub fn background_pixel(mut self, p: c_ulong) -> Self {
        self.background_pixel = p;
        self
    }
}
