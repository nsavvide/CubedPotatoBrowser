use std::sync::{Arc, Mutex};

use crate::handlers::resource_request::PResourceRequestHandler;
use adblock::Engine;
use cef::rc::{Rc, RcImpl};
use cef::{ImplRequestHandler, ResourceRequestHandler, WrapRequestHandler};

#[derive(Clone)]
pub struct PRequestHandler {
    engine: Arc<Mutex<Engine>>,
    pub object: *mut RcImpl<cef_dll_sys::_cef_client_t, Self>,
}

impl PRequestHandler {
    pub fn new(engine: Arc<Mutex<Engine>>) -> Self {
        Self {
            engine,
            object: std::ptr::null_mut(),
        }
    }
}

impl WrapRequestHandler for PRequestHandler {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_dll_sys::_cef_request_handler_t, Self>) {
        self.object = object.cast();
    }
}

impl ImplRequestHandler for PRequestHandler {
    fn resource_request_handler(
        &self,
        _browser: Option<&mut cef::Browser>,
        _frame: Option<&mut cef::Frame>,
        _request: Option<&mut cef::Request>,
        _is_navigation: i32,
        _is_download: i32,
        _request_initiator: Option<&cef::CefString>,
        _disable_default_handling: Option<&mut i32>,
    ) -> Option<ResourceRequestHandler> {
        Some(ResourceRequestHandler::new(PResourceRequestHandler::new(
            self.engine.clone(),
        )))
    }

    fn get_raw(&self) -> *mut cef_dll_sys::_cef_request_handler_t {
        self.object.cast()
    }
}

impl Rc for PRequestHandler {
    fn as_base(&self) -> &cef_dll_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}
