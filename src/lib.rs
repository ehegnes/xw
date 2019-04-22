//! A safe, high-level builder-pattern wrapper around [`x11`].

#![allow(dead_code)]

extern crate libc;
extern crate x11;

pub mod attributes;
pub mod visualinfo;
pub mod window;
pub mod xstr;

use libc::c_ulong;
use std::ffi::CString;
use std::ptr;
use x11::xlib;

use attributes::WindowAttributes;
use visualinfo::VisualInfo;
use window::Window;
use xstr::XStr;

#[derive(Clone)]
struct XBuilder {
    display: *mut xlib::Display,
    visual_info: *mut xlib::XVisualInfo,
    colormap: c_ulong,
    attributes: *mut xlib::XSetWindowAttributes,
    window: xlib::Window,
    gc: xlib::GC,
}

impl Default for XBuilder {
    fn default() -> Self {
        XBuilder {
            display: unsafe { xlib::XOpenDisplay(ptr::null()) },
            visual_info: unsafe {
                libc::malloc(std::mem::size_of::<xlib::XVisualInfo>()) as *mut xlib::XVisualInfo
            },
            attributes: unsafe {
                libc::malloc(std::mem::size_of::<xlib::XSetWindowAttributes>())
                    as *mut xlib::XSetWindowAttributes
            },
            window: xlib::Window::default(),
            colormap: 0,
            gc: ptr::null_mut(),
        }
    }
}

impl XBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Allows specification of a display name
    pub fn with_display(display_name: &str) -> XBuilder {
        XBuilder {
            display: unsafe { xlib::XOpenDisplay(XStr(display_name).into()) },
            ..Default::default()
        }
    }

    /// Sets [`XVisualInfo`](x11::xlib::XVisualInfo)
    fn visual(self, visual_info: VisualInfo) -> Self {
        unsafe {
            xlib::XMatchVisualInfo(
                self.display,
                self.default_screen(),
                visual_info.depth,
                visual_info.class as i32,
                self.visual_info,
            );
        }
        self
    }

    // XXX: Should this be abstracted?
    fn colormap(mut self) -> Self {
        unsafe {
            self.colormap = xlib::XCreateColormap(
                self.display,
                self.default_root_window(),
                (*self.visual_info).visual,
                xlib::AllocNone,
            );
        }
        self
    }

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
    fn window(&mut self, window: Window) {
        unsafe {
            self.window = xlib::XCreateWindow(
                self.display,
                self.default_root_window(),
                window.x,
                window.y,
                window.width,
                window.height,
                window.border_width,
                (*self.visual_info).depth,
                window.class as u32,
                (*self.visual_info).visual,
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
    fn with_window(mut self, window: Window) -> Self {
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
    fn default_screen(&self) -> i32 {
        unsafe { xlib::XDefaultScreen(self.display) }
    }

    /// Wait for [`x11::xlib::MayNotify`] and flush to render updates
    fn flush(&self) {
        unsafe {
            // Wait for the MapNotify event
            loop {
                let mut e = xlib::XEvent { pad: [0; 24] };
                xlib::XNextEvent(self.display, &mut e);
                if e.type_ == xlib::MapNotify {
                    break;
                }
            }
            xlib::XFlush(self.display);
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

impl Drop for XBuilder {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.visual_info as *mut libc::c_void);
            libc::free(self.attributes as *mut libc::c_void);
            if self.colormap != 0 {
                xlib::XFreeColormap(self.display, self.colormap);
            }
            if self.gc != ptr::null_mut() {
                xlib::XFreeGC(self.display, self.gc);
            }
            xlib::XCloseDisplay(self.display);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        XBuilder::default();
    }

    #[test]
    fn visual() {
        XBuilder::default().visual(VisualInfo::default());
    }

    #[test]
    fn colormap() {
        XBuilder::default().visual(VisualInfo::default()).colormap();
    }

    #[test]
    fn attributes() {
        XBuilder::default()
            .visual(VisualInfo::default())
            .colormap()
            .attributes();
    }

    //#[test]
    fn with_window() {
        XBuilder::default()
            .visual(VisualInfo::default())
            .colormap()
            .attributes()
            .with_window(Window::default().width(100).height(100))
            .flush();
    }

    //#[test]
    fn flush() {
        XBuilder::default()
            .visual(VisualInfo::default())
            .colormap()
            .attributes()
            .with_window(Window::default().x(100).y(100).width(200).height(200))
            .flush();
    }

    //#[test]
    fn separate_window() {
        let mut x = XBuilder::default()
            .visual(VisualInfo::default())
            .colormap()
            .attributes();
        let window = Window::default().x(100).y(100).width(200).height(200);
        x.window(window);
        x.flush();
    }

    #[test]
    fn window_name() {
        let mut x = XBuilder::default()
            .visual(VisualInfo::default())
            .colormap()
            .attributes()
            .with_window(Window::default().name("Test Name"))
            .flush();
    }
}
