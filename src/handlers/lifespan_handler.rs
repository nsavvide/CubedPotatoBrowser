use cef::rc::{Rc, RcImpl};
use cef::{Browser, ImplLifeSpanHandler, LifeSpanHandler, WrapLifeSpanHandler};
use std::sync::{Arc, Mutex};

pub struct PLifeSpanHandler {
    object: *mut RcImpl<cef_dll_sys::_cef_life_span_handler_t, Self>,
    pub browser: Arc<Mutex<Option<Browser>>>,
}

impl PLifeSpanHandler {
    pub fn new(browser: Arc<Mutex<Option<Browser>>>) -> LifeSpanHandler {
        LifeSpanHandler::new(Self {
            object: std::ptr::null_mut(),
            browser,
        })
    }
}

impl Rc for PLifeSpanHandler {
    fn as_base(&self) -> &cef_dll_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl WrapLifeSpanHandler for PLifeSpanHandler {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_dll_sys::_cef_life_span_handler_t, Self>) {
        self.object = object;
    }
}

impl Clone for PLifeSpanHandler {
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

impl ImplLifeSpanHandler for PLifeSpanHandler {
    fn get_raw(&self) -> *mut cef_dll_sys::_cef_life_span_handler_t {
        self.object.cast()
    }

    fn on_after_created(&self, browser: Option<&mut Browser>) {
        println!("on_after_created called");
        if let Some(browser) = browser {
            let b = Browser::clone(browser);
            let mut lock = self.browser.lock().unwrap();
            *lock = Some(b);
            println!("Browser instance saved in Arc (on_after_created)");
        }
    }
}
