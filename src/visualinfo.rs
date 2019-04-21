//! Abstraction for [`XVisualInfo`](x11::xlib::XVisualInfo)

#[repr(i32)]
/// Abstraction of visual classes
pub enum VisualClass {
    StaticGray,  // 0
    GrayScale,   // 1
    StaticColor, // 2
    PseudoColor, // 3
    TrueColor,   // 4
    DirectColor, // 5
}

pub struct VisualInfo {
    pub depth: i32,
    pub class: VisualClass,
}

impl Default for VisualInfo {
    fn default() -> Self {
        VisualInfo {
            depth: 32,
            class: VisualClass::TrueColor,
        }
    }
}

impl VisualInfo {
    pub fn depth(mut self, depth: i32) -> Self {
        self.depth = depth;
        self
    }

    pub fn class(mut self, class: VisualClass) -> Self {
        self.class = class;
        self
    }
}
