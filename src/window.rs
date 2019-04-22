//! Abstraction for [`x11::xlib::XCreateWindow`]

use x11::xlib::{CWBackPixel, CWBorderPixel, CWColormap};

#[repr(u32)]
pub enum WindowClass {
    CopyFromParent, // 0 XXX: unsupported in x11-rs
    InputOutput,    // 1
    InputOnly,      // 2
}

/// Defines a window to be provided to XBuilder
///
/// XXX: should these types be from `libc`?
pub struct Window {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub border_width: u32,
    pub class: WindowClass,
    pub valuemask: u64,
}

impl Default for Window {
    fn default() -> Self {
        Window {
            name: String::default(),
            x: 0,
            y: 0,
            width: 1,
            height: 1,
            border_width: 1,
            class: WindowClass::InputOutput,
            valuemask: CWColormap | CWBorderPixel | CWBackPixel,
        }
    }
}

impl Window {
    /// Creates a new [`Window`] with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the window's name
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = name.into();
        self
    }

    /// Set the window's x-coordinate
    pub fn x(mut self, x: i32) -> Self {
        self.x = x;
        self
    }

    /// Set the window's y-coordinate
    pub fn y(mut self, y: i32) -> Self {
        self.y = y;
        self
    }

    /// Set the window's width
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Set the window's height
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    /// Set the window's border width
    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = border_width;
        self
    }

    /// Set the window's class
    ///
    /// # Caveats
    /// - `WindowClass::InputOnly` requires that `Window::border_width` be `0`.
    /// - `WindowClass::InputOnly` requires that `VisualInfo::depth` be `0`.
    pub fn class(mut self, class: WindowClass) -> Self {
        self.class = class;
        self
    }

    /// Set the window's valuemask
    ///
    /// # Values
    /// Appropriate from `x11::xlib` are as follows:
    /// ```c
    /// #define   CWBackPixmap                (1L<<0)
    /// #define   CWBackPixel                 (1L<<1)
    /// #define   CWBorderPixmap              (1L<<2)
    /// #define   CWBorderPixel               (1L<<3)
    /// #define   CWBitGravity                (1L<<4)
    /// #define   CWWinGravity                (1L<<5)
    /// #define   CWBackingStore              (1L<<6)
    /// #define   CWBackingPlanes             (1L<<7)
    /// #define   CWBackingPixel              (1L<<8)
    /// #define   CWOverrideRedirect          (1L<<9)
    /// #define   CWSaveUnder                 (1L<<10)
    /// #define   CWEventMask                 (1L<<11)
    /// #define   CWDontPropagate             (1L<<12)
    /// #define   CWColormap                  (1L<<13)
    /// #define   CWCursor                    (1L<<14)
    /// ```
    pub fn valuemask(mut self, valuemask: u64) -> Self {
        self.valuemask = valuemask;
        self
    }
}
