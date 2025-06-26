use cef::{CefStringUserfreeUtf16};
use cef_dll_sys::_cef_string_utf16_t;
use std::slice;

pub fn string_utf16_to_utf8(cef_string: &CefStringUserfreeUtf16) -> String {
    if let Some(inner) = Option::<&_cef_string_utf16_t>::from(cef_string) {
        if !inner.str_.is_null() && inner.length > 0 {
            unsafe {
                let slice = slice::from_raw_parts(inner.str_, inner.length);
                return String::from_utf16_lossy(slice);
            }
        }
    }
    String::new()
}
