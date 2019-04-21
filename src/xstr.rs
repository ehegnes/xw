use libc::c_char;
use std::ffi::CString;

pub struct XStr<'a>(pub &'a str);

impl<'a> Into<*const c_char> for XStr<'a> {
    fn into(self) -> *const c_char {
        CString::new(self.0).unwrap().as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xstr() {
        let s = XStr("abc");
    }
}
