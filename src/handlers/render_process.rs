use std::{
    ffi::c_int,
    sync::{Arc, Mutex},
};

use cef::{
    process_message_create, rc::{Rc, RcImpl}, Browser, CefString, Domnode, Frame, ImplDomnode, ImplFrame,
    ImplListValue, ImplProcessMessage, ImplRenderProcessHandler, ProcessId, RenderProcessHandler, WrapRenderProcessHandler,
};
use cef_dll_sys::cef_process_id_t;

use crate::browser::keybinds::KeybindingManager;

#[derive(Clone)]
pub struct PRenderHandler {
    pub keybindings: Arc<Mutex<KeybindingManager>>,
    pub object: *mut cef_dll_sys::_cef_resource_request_handler_t,
}

impl PRenderHandler {
    pub fn new() -> RenderProcessHandler {
        let handler = Self {
            keybindings: Arc::new(Mutex::new(KeybindingManager::new())),
            object: std::ptr::null_mut(),
        };

        RenderProcessHandler::new(handler)
    }
}

impl WrapRenderProcessHandler for PRenderHandler {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_dll_sys::_cef_render_process_handler_t, Self>) {
        self.object = object.cast();
    }
}

impl Rc for PRenderHandler {
    fn as_base(&self) -> &cef_dll_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.base)
        }
    }
}

impl ImplRenderProcessHandler for PRenderHandler {
    fn on_focused_node_changed(
        &self,
        _browser: Option<&mut Browser>,
        frame: Option<&mut Frame>,
        node: Option<&mut Domnode>,
    ) {
        let is_editing = node
            .map(|n| n.is_editable() == 1 || n.is_form_control_element() == 1)
            .unwrap_or(false);

        println!("Focused node changed: is_editing={}", is_editing);

        if let Some(frame) = frame {
            let name = CefString::from("UpdateInsertMode");
            if let Some(mut message) = process_message_create(Some(&name)) {
                if let Some(list) = message.argument_list() {
                    list.set_bool(0, is_editing as c_int);
                }

                let pid = ProcessId::from(cef_process_id_t::PID_BROWSER);
                frame.send_process_message(pid, Some(&mut message));
            }
        }
    }

    fn get_raw(&self) -> *mut cef_dll_sys::_cef_render_process_handler_t {
        self.object.cast()
    }
}
