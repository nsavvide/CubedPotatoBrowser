// use crate::browser::client::PClient;
// use crate::browser::window::create_main_window;
use cef::Window;
use cef::{rc::*, *};
use std::sync::{Arc, Mutex};

pub struct PBrowserProcessHandler {
    pub object: *mut RcImpl<cef_dll_sys::cef_browser_process_handler_t, Self>,
    pub windows: Arc<Mutex<Vec<Window>>>,
}

impl PBrowserProcessHandler {
    pub fn new(windows: Arc<Mutex<Vec<Window>>>) -> BrowserProcessHandler {
        BrowserProcessHandler::new(Self {
            object: std::ptr::null_mut(),
            windows,
        })
    }
}

impl Rc for PBrowserProcessHandler {
    fn as_base(&self) -> &cef_dll_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl WrapBrowserProcessHandler for PBrowserProcessHandler {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_dll_sys::_cef_browser_process_handler_t, Self>) {
        self.object = object;
    }
}

impl Clone for PBrowserProcessHandler {
    fn clone(&self) -> Self {
        let object = unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
            self.object
        };

        let windows = self.windows.clone();

        Self { object, windows }
    }
}

impl ImplBrowserProcessHandler for PBrowserProcessHandler {
    fn get_raw(&self) -> *mut cef_dll_sys::_cef_browser_process_handler_t {
        self.object.cast()
    }

    fn on_context_initialized(&self) {
        println!("cef context initialized");
    }
}
