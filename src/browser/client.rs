use crate::handlers::keyboard::PKeyboardHandler;
use crate::handlers::lifespan_handler::PLifeSpanHandler;
use crate::handlers::request::PRequestHandler;
use adblock::Engine;
use cef::{rc::*, Client, *};
use std::sync::Arc;
use std::sync::Mutex;

pub struct PClient {
    object: *mut RcImpl<cef_dll_sys::_cef_client_t, Self>,
    pub browser: Arc<Mutex<Option<Browser>>>,
    pub keyboard_handler: KeyboardHandler,
    pub adblock_engine: Arc<Mutex<Engine>>,
}

impl PClient {
    pub fn new(browser: Arc<Mutex<Option<Browser>>>, engine: Arc<Mutex<Engine>>) -> Client {
        let keyboard_handler = KeyboardHandler::new(PKeyboardHandler::new(browser.clone()));
        Client::new(Self {
            object: std::ptr::null_mut(),
            browser,
            keyboard_handler,
            adblock_engine: engine,
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
            keyboard_handler: self.keyboard_handler.clone(),
            adblock_engine: self.adblock_engine.clone(),
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
        Some(self.keyboard_handler.clone())
    }

    fn life_span_handler(&self) -> Option<LifeSpanHandler> {
        Some(PLifeSpanHandler::new(self.browser.clone()))
    }

    fn request_handler(&self) -> Option<RequestHandler> {
        Some(RequestHandler::new(PRequestHandler::new(
            self.adblock_engine.clone(),
        )))
    }
}
