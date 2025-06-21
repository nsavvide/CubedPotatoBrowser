use crate::handlers::keyboard::PKeyboardHandler;
use crate::handlers::lifespan_handler::PLifeSpanHandler;
use cef::{rc::*, Client, *};
use std::sync::Arc;
use std::sync::Mutex;

pub struct PClient {
    object: *mut RcImpl<cef_dll_sys::_cef_client_t, Self>,
    pub browser: Arc<Mutex<Option<Browser>>>,
}

impl PClient {
    pub fn new(browser: Arc<Mutex<Option<Browser>>>) -> Client {
        Client::new(Self {
            object: std::ptr::null_mut(),
            browser,
        })
    }
}

impl WrapClient for PClient {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_dll_sys::_cef_client_t, Self>) {
        self.object = object;
    }
}

impl Clone for PClient {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
        }

        Self {
            object: self.object,
            browser: self.browser.clone(),
        }
    }
}

impl Rc for PClient {
    fn as_base(&self) -> &cef_dll_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl ImplClient for PClient {
    fn get_raw(&self) -> *mut cef_dll_sys::_cef_client_t {
        self.object.cast()
    }

    fn keyboard_handler(&self) -> Option<KeyboardHandler> {
        Some(KeyboardHandler::new(PKeyboardHandler::new(
            self.browser.clone(),
        )))
    }

    fn life_span_handler(&self) -> Option<LifeSpanHandler> {
        Some(PLifeSpanHandler::new(self.browser.clone()))
    }
}
