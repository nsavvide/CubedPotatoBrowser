use adblock::request::Request as AdblockRequest;
use adblock::Engine;
use cef::rc::RcImpl;
use cef::{
    Browser, Callback, ImplFrame, ImplRequest, ImplResourceRequestHandler, Request, ReturnValue, WrapResourceRequestHandler,
};
use cef_dll_sys::cef_return_value_t;
use std::sync::{Arc, Mutex};

use crate::utils::infer_request_type::infer_request_type;
use crate::utils::string::string_utf16_to_utf8;

#[derive(Clone)]
pub struct PResourceRequestHandler {
    engine: Arc<Mutex<Engine>>,
    pub object: *mut cef_dll_sys::_cef_resource_request_handler_t,
}

impl PResourceRequestHandler {
    pub fn new(engine: Arc<Mutex<Engine>>) -> Self {
        Self {
            engine,
            object: std::ptr::null_mut(),
        }
    }
}

impl cef::rc::Rc for PResourceRequestHandler {
    fn as_base(&self) -> &cef_dll_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.base)
        }
    }
}

impl ImplResourceRequestHandler for PResourceRequestHandler {
    fn on_before_resource_load(
        &self,
        _browser: Option<&mut Browser>,
        _frame: Option<&mut cef::Frame>,
        request: Option<&mut Request>,
        _callback: Option<&mut Callback>,
    ) -> ReturnValue {
        if let Some(req) = request {
            let url_str = string_utf16_to_utf8(&req.url());

            let source_url = _frame
                .map(|f| string_utf16_to_utf8(&f.url()))
                .unwrap_or_default();

            let request_type_str = infer_request_type(&url_str);

            match AdblockRequest::new(&url_str, &source_url, request_type_str) {
                Ok(adblock_req) => {
                    let result = self
                        .engine
                        .lock()
                        .unwrap()
                        .check_network_request(&adblock_req);

                    if result.matched {
                        println!("[Adblock] BLOCKED: {}", adblock_req.url);
                        return ReturnValue::from(cef_return_value_t::RV_CANCEL);
                    } else {
                        println!("[Adblock] ALLOWED: {}", adblock_req.url);
                    }
                }
                Err(err) => {
                    eprintln!("[Adblock] Failed to build request for {}: {}", url_str, err);
                }
            }
        }

        ReturnValue::from(cef_return_value_t::RV_CONTINUE)
    }

    fn get_raw(&self) -> *mut cef_dll_sys::_cef_resource_request_handler_t {
        self.object
    }
}

impl WrapResourceRequestHandler for PResourceRequestHandler {
    fn wrap_rc(&mut self, rc: *mut RcImpl<cef_dll_sys::_cef_resource_request_handler_t, Self>) {
        self.object = rc.cast();
    }
}
