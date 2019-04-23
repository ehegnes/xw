use std::collections::HashMap;
use x11::xlib;

pub type Colors = HashMap<&'static str, *mut xlib::XColor>;
