use std::collections::HashMap;
use x11::xlib;

pub type Colors = HashMap<String, *mut xlib::XColor>;
