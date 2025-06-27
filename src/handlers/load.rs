use cef::{
    rc::*, Browser, CefString, Frame, ImplFrame, ImplLoadHandler, LoadHandler, WrapLoadHandler,
};
use std::ptr::null_mut;

#[derive(Clone)]
pub struct PLoadHandler {
    object: *mut RcImpl<cef_dll_sys::_cef_load_handler_t, Self>,
}

impl PLoadHandler {
    pub fn new() -> LoadHandler {
        LoadHandler::new(Self { object: null_mut() })
    }
}

impl Rc for PLoadHandler {
    fn as_base(&self) -> &cef_dll_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl WrapLoadHandler for PLoadHandler {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_dll_sys::_cef_load_handler_t, Self>) {
        self.object = object;
    }
}

impl ImplLoadHandler for PLoadHandler {
    fn get_raw(&self) -> *mut cef_dll_sys::_cef_load_handler_t {
        self.object.cast()
    }

    fn on_load_end(
        &self,
        _browser: Option<&mut Browser>,
        frame: Option<&mut Frame>,
        _http_status_code: i32,
    ) {
        if let Some(frame) = frame {
            if frame.is_main() == 1 {
                let load_command_bar =
                    CefString::from(include_str!("../assets/scripts/command_bar/command_bar.js"));

                frame.execute_java_script(Some(&load_command_bar), None, 0);

                let remove_hints =
                    CefString::from(include_str!("../assets/scripts/overlay/remove_overlay.js"));
                frame.execute_java_script(Some(&remove_hints), None, 0);
            }
        }
    }
}
