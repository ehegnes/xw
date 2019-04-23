//! A safe, high-level builder-pattern wrapper around [`x11`].

#![allow(dead_code)]

extern crate libc;
extern crate x11;

pub mod attributes;
pub mod color;
pub mod draw;
pub mod visualinfo;
pub mod window;

use libc::{c_int, c_ulong};
use std::ffi::CString;
use std::{mem, ptr};
use x11::xlib;

use attributes::WindowAttributes;
use color::Colors;
use draw::Drawable;
use visualinfo::VisualInfo;
use window::Window;

#[derive(Clone)]
struct XBuilder {
    display: *mut xlib::Display,
    visual_info: xlib::XVisualInfo,
    colormap: c_ulong,
    attributes: *mut xlib::XSetWindowAttributes,
    window: xlib::Window,
    gc: xlib::GC,
    colors: Colors,
}

impl Default for XBuilder {
    fn default() -> Self {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            let screen_num = xlib::XDefaultScreen(display);

            let default_depth = xlib::XDefaultDepth(display, screen_num);
            let default_visual_class = (*xlib::XDefaultVisual(display, screen_num)).class;
            let mut visual_info = mem::uninitialized();
            xlib::XMatchVisualInfo(
                display,
                screen_num,
                default_depth,
                default_visual_class,
                &mut visual_info,
            );

            println!("default_depth: {:?}", default_depth);
            println!("default_visual_class: {:?}", default_visual_class);

            let attributes = libc::malloc(std::mem::size_of::<xlib::XSetWindowAttributes>())
                as *mut xlib::XSetWindowAttributes;
            let _attributes = WindowAttributes::from_display(display);
            (*attributes).border_pixel = _attributes.border_pixel;
            (*attributes).background_pixel = _attributes.background_pixel;

            XBuilder {
                display,
                visual_info,
                attributes,
                colormap: xlib::XDefaultColormap(display, screen_num),
                window: xlib::Window::default(),
                gc: ptr::null_mut(),
                colors: Colors::default(),
            }
        }
    }
}

impl XBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_color(mut self, name: &'static str) -> Self {
        unsafe {
            let mut color = mem::uninitialized();
            xlib::XAllocNamedColor(
                self.display,
                self.colormap,
                // XXX: this leaks memory because of `into_raw()`, but
                //      `as_ptr()` doesn't give string ownership to X, which
                //      seems to be required in `XAllocNamedColor()`.
                CString::new(name).unwrap().into_raw(),
                &mut color,
                &mut color,
            );
            println!("added color: {:?}", color);
            self.colors.insert(name, &mut color);
            self
        }
    }

    /// Set the global foreground color
    /// TODO: Extend this to be per-window
    pub fn set_foreground(self, name: &'static str) -> Self {
        let color = *self.colors.get(name).expect("Could not find named color.");
        unsafe {
            println!("using color as foreground: {:?}", *color);
            xlib::XSetForeground(self.display, self.gc, (*color).pixel);
        }
        self
    }

    /// Set the global background color
    pub fn set_background(self, name: &'static str) {
        let color = *self.colors.get(name).expect("Could not find named color.");
        unsafe {
            xlib::XSetBackground(self.display, self.gc, (*color).pixel);
        }
    }

    /// Sets [`XVisualInfo`](x11::xlib::XVisualInfo)
    fn visual(mut self, visual_info: VisualInfo) -> Self {
        unsafe {
            xlib::XMatchVisualInfo(
                self.display,
                self.default_screen(),
                visual_info.depth,
                visual_info.class as i32,
                &mut self.visual_info,
            );
        }
        self
    }

    /// TODO: Abstract this or remove it
    fn colormap(mut self) -> Self {
        unsafe {
            self.colormap = xlib::XCreateColormap(
                self.display,
                self.default_root_window(),
                self.visual_info.visual,
                xlib::AllocNone,
            );
        }
        self
    }

    /// TODO: Abstract this
    fn attributes(mut self) -> Self {
        let attributes = WindowAttributes::from_display(self.display);
        unsafe {
            (*self.attributes).colormap = self.colormap;
            (*self.attributes).border_pixel = attributes.border_pixel;
            (*self.attributes).background_pixel = attributes.background_pixel;
        }
        self
    }

    /// Allows defining and adding a window separately.
    pub fn window(&mut self, window: Window) {
        unsafe {
            self.window = xlib::XCreateWindow(
                self.display,
                self.default_root_window(),
                window.x,
                window.y,
                window.width,
                window.height,
                window.border_width,
                self.visual_info.depth,
                window.class as u32,
                self.visual_info.visual,
                window.valuemask,
                self.attributes,
            );

            // Set name
            // XXX: Check safer methods rather than `into_raw`
            let name = CString::new(window.name).unwrap().into_raw();
            let class_hints = Box::into_raw(Box::new(xlib::XClassHint {
                res_name: name,
                res_class: name,
            }));
            xlib::XSetClassHint(self.display, self.window, class_hints);
            xlib::XStoreName(self.display, self.window, name);
            CString::from_raw(name);
            Box::from_raw(class_hints);

            xlib::XSelectInput(self.display, self.window, xlib::StructureNotifyMask);
            xlib::XMapWindow(self.display, self.window);

            // TODO: abstract
            self.gc = xlib::XCreateGC(self.display, self.window, 0, ptr::null_mut());
        }
    }

    /// Defines a window within the builder pattern
    pub fn with_window(mut self, window: Window) -> Self {
        self.window(window);
        self
    }

    /// Alias for [`x11::xlib::XDefaultRootWindow`]
    fn default_root_window(&self) -> c_ulong {
        unsafe { xlib::XDefaultRootWindow(self.display) }
    }

    /// Alias for [`x11::xlib::XDefaultScreenOfDisplay`]
    fn default_screen_of_display(&self) -> *mut xlib::Screen {
        unsafe { xlib::XDefaultScreenOfDisplay(self.display) }
    }

    /// Alias for [`x11::xlib::XDefaultScreen`]
    fn default_screen(&self) -> c_int {
        unsafe { xlib::XDefaultScreen(self.display) }
    }

    /// Flush to display
    pub fn flush(&self) {
        unsafe {
            xlib::XFlush(self.display);
        }

        // XXX: temporary for debugging purposes
        std::thread::sleep(std::time::Duration::from_secs(3));
    }

    pub fn draw<T: Drawable>(self, drawable: T) -> Self {
        // Wait for the MapNotify event
        unsafe {
            loop {
                let mut e = xlib::XEvent { pad: [0; 24] };
                xlib::XNextEvent(self.display, &mut e);
                if e.type_ == xlib::MapNotify {
                    break;
                }
            }
        }
        drawable.draw(self.display, self.window, self.gc);
        self
    }
}

impl Drop for XBuilder {
    fn drop(&mut self) {
        unsafe {
            // XXX: Learn why this doesn't work
            /*
             *xlib::XFreeColors(
             *    self.display,
             *    self.colormap,
             *    self.colors
             *        .values()
             *        .map(|&x| (*x).pixel)
             *        .collect::<Vec<c_ulong>>()
             *        .as_mut_ptr(),
             *    self.colors.len() as i32,
             *    0,
             *);
             */
            libc::free(self.attributes as *mut libc::c_void);
            xlib::XFreeColormap(self.display, self.colormap);
            if !ptr::eq(self.gc, ptr::null_mut()) {
                xlib::XFreeGC(self.display, self.gc);
            }
            xlib::XCloseDisplay(self.display);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn new() {
        XBuilder::new();
    }

    //#[test]
    fn visual() {
        XBuilder::new().visual(VisualInfo::new());
    }

    //#[test]
    fn colormap() {
        XBuilder::new().colormap();
    }

    //#[test]
    fn attributes() {
        XBuilder::new().attributes();
    }

    //#[test]
    fn with_window() {
        XBuilder::new().with_window(Window::new());
    }

    //#[test]
    fn flush() {
        XBuilder::new().with_window(Window::new()).flush();
    }

    //#[test]
    fn separate_window() {
        let mut x = XBuilder::new();
        let window = Window::new();
        x.window(window);
        x.flush();
    }

    //#[test]
    fn window_name() {
        XBuilder::new()
            .with_window(Window::new().name("Test Name"))
            .flush();
    }
}
