use cef::{rc::*, *};
use crate::handlers::browser_process::PBrowserProcessHandler;
use std::sync::{Arc, Mutex};

pub struct PApp {
    pub object: *mut RcImpl<cef_dll_sys::_cef_app_t, Self>,
    pub windows: Arc<Mutex<Vec<Window>>>,
}

impl PApp {
    pub fn new(windows: Arc<Mutex<Vec<Window>>>) -> App {
        App::new(Self {
            object: std::ptr::null_mut(),
            windows,
        })
    }
}

impl WrapApp for PApp {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_dll_sys::_cef_app_t, Self>) {
        self.object = object;
    }
}

impl Clone for PApp {
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

impl Rc for PApp {
    fn as_base(&self) -> &cef_dll_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl ImplApp for PApp {
    fn get_raw(&self) -> *mut cef_dll_sys::_cef_app_t {
        self.object.cast()
    }

    fn browser_process_handler(&self) -> Option<BrowserProcessHandler> {
        Some(PBrowserProcessHandler::new(self.windows.clone()))
    }
}
