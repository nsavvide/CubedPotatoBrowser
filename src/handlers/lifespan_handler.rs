use cef::rc::{Rc, RcImpl};
use cef::{
    Browser, ImplLifeSpanHandler, ImplView, LifeSpanHandler, Window, WrapLifeSpanHandler
};
use std::sync::{Arc, Mutex};

pub struct PLifeSpanHandler {
    object: *mut RcImpl<cef_dll_sys::_cef_life_span_handler_t, Self>,
    pub browser: Arc<Mutex<Option<Browser>>>,
    pub window: Arc<Mutex<Option<Window>>>,
    pub windows: Arc<Mutex<Vec<Window>>>,
}

impl PLifeSpanHandler {
    pub fn new(
        browser: Arc<Mutex<Option<Browser>>>,
        window: Arc<Mutex<Option<Window>>>,
        windows: Arc<Mutex<Vec<Window>>>,
    ) -> LifeSpanHandler {
        LifeSpanHandler::new(Self {
            object: std::ptr::null_mut(),
            browser,
            window,
            windows,
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
            window: self.window.clone(),
            windows: self.windows.clone(),
        }
    }
}

impl ImplLifeSpanHandler for PLifeSpanHandler {
    fn get_raw(&self) -> *mut cef_dll_sys::_cef_life_span_handler_t {
        self.object.cast()
    }

    fn on_after_created(&self, browser: Option<&mut Browser>) {
        if let Some(browser) = browser {
            let b = Browser::clone(browser);
            let mut lock = match self.browser.lock() {
                Ok(guard) => guard,
                Err(poisoned) => {
                    eprintln!("Mutex lock poisoned, recovering...");
                    poisoned.into_inner()
                }
            };
            *lock = Some(b);
            println!("Browser instance saved in Arc (on_after_created)");
        }
    }

    fn on_before_close(&self, _browser: Option<&mut Browser>) {
        if let Some(window) = self.window.lock().unwrap().as_ref() {
            let id = window.id(); 
            println!("[on_before_close] Closing window with ID: {}", id);
            self.windows.lock().unwrap().retain(|w| w.id() != id);
        }
    }
}
