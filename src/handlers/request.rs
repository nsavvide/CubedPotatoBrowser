use adblock::{Engine, Request, RequestType};
use cef::{Browser, Frame, Request, RequestHandler, ResourceRequestHandler, UrlRequest, WrapRequestHandler};
use std::sync::{Arc, Mutex};

pub struct PRequestHandler {
    engine: Arc<Mutex<Engine>>,
}

impl PRequestHandler {
    pub fn new(engine: Arc<Mutex<Engine>>) -> Self {
        Self { engine }
    }
}

impl WrapRequestHandler for PRequestHandler {
    fn resource_request_handler(
        &self,
        _browser: Option<&mut Browser>,
        _frame: Option<&mut Frame>,
        request: Option<&mut Request>,
        _is_navigation: i32,
        _is_download: i32,
        _request_initiator: Option<&cef::CefString>,
        disable_default_handling: Option<&mut i32>,
    ) -> Option<ResourceRequestHandler> {
        if let Some(req) = request {
            let url = req.url().unwrap_or_default();

            let adblock_req = AdblockRequest {
                url: url.clone(),
                source_url: "".to_string(), // You can fill this in later
                request_type: RequestType::Other,
                initiator_domain: None,
                is_third_party: true,
            };

            let blocked = self
                .engine
                .lock()
                .unwrap()
                .check_network_request(&adblock_req)
                .matched();

            if blocked {
                println!("[Adblock] BLOCKED: {}", url);
                if let Some(flag) = disable_default_handling {
                    *flag = 1; // disables default handling
                }
                return None;
            }
        }

        None
    }

    fn wrap_rc(&mut self, object: *mut cef::rc::RcImpl<cef_dll_sys::_cef_request_handler_t, Self>) {
        todo!()
    }
}
