use cef::CefStringUserfreeUtf16;
use cef_dll_sys::_cef_string_utf16_t;

pub fn string_utf16_to_utf8(cef_string: &CefStringUserfreeUtf16) -> String {
    // SAFETY: We rely on the From impl to get access to the inner pointer.
    // Then, we reconstruct the _cef_string_utf16_t and read the string safely.
    let raw = cef_string as *const CefStringUserfreeUtf16 as *const _cef_string_utf16_t;
    unsafe {
        let str_ptr = (*raw).str_;
        let len = (*raw).length;
        if str_ptr.is_null() {
            return String::new();
        }
        let slice = std::slice::from_raw_parts(str_ptr, len);
        String::from_utf16_lossy(slice)
    }
}
