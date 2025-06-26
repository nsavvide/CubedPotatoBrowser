use crate::handlers::keyboard::KeyboardHandlerBundle;
use crate::handlers::keyboard::PKeyboardHandler;
use crate::handlers::lifespan_handler::PLifeSpanHandler;
use crate::handlers::request::PRequestHandler;
use adblock::Engine;
use cef::{rc::*, Client, *};
use std::ffi::c_int;
use std::sync::Arc;
use std::sync::Mutex;

pub struct PClient {
    object: *mut RcImpl<cef_dll_sys::_cef_client_t, Self>,
    pub browser: Arc<Mutex<Option<Browser>>>,
    pub keyboard: Option<KeyboardHandlerBundle>,
    pub adblock_engine: Arc<Mutex<Engine>>,
}

impl PClient {
    pub fn new(browser: Arc<Mutex<Option<Browser>>>, engine: Arc<Mutex<Engine>>) -> Client {

        let keyboard_impl = PKeyboardHandler::new(browser.clone());

        Client::new(Self {
            object: std::ptr::null_mut(),
            browser,
            keyboard: Some(KeyboardHandlerBundle {
                handler: None,
                implementation: keyboard_impl,
            }),
            adblock_engine: engine,
        })
    }
}

impl WrapClient for PClient {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_dll_sys::_cef_client_t, Self>) {
        self.object = object;

        if let Some(bundle) = &mut self.keyboard {
            bundle.implementation.wrap_rc(object.cast());
            bundle.handler = Some(KeyboardHandler::new(bundle.implementation.clone()));
        } 
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
            keyboard: self.keyboard.as_ref().map(|bundle| {
                KeyboardHandlerBundle {
                    handler: bundle.handler.clone(),
                    implementation: bundle.implementation.clone(),
                }
            }),
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
        self.keyboard.as_ref().and_then(|b| b.handler.clone())
    }

    fn life_span_handler(&self) -> Option<LifeSpanHandler> {
        Some(PLifeSpanHandler::new(self.browser.clone()))
    }

    fn request_handler(&self) -> Option<RequestHandler> {
        Some(RequestHandler::new(PRequestHandler::new(
            self.adblock_engine.clone(),
        )))
    }

    fn on_process_message_received(
        &self,
        _browser: Option<&mut Browser>,
        _frame: Option<&mut Frame>,
        _source_process: ProcessId,
        message: Option<&mut ProcessMessage>,
    ) -> c_int {
        if let Some(msg) = message {
            if CefString::from(&msg.name()).to_string() == "UpdateInsertMode" {
                if let Some(args) = msg.argument_list() {
                    let is_editing = args.bool(0);
                    if let Some(bundle) = &self.keyboard {
                        bundle.implementation.set_insert_mode(is_editing != 0);
                    }
                    return 1;
                }
            }
        }
        0
    }
}
