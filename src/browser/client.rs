use cef::{Client, rc::*, *};

pub struct PClient(*mut RcImpl<cef_dll_sys::_cef_client_t, Self>);

impl PClient {
    pub fn new() -> Client {
        Client::new(Self(std::ptr::null_mut()))
    }
}

impl WrapClient for PClient {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_dll_sys::_cef_client_t, Self>) {
        self.0 = object;
    }
}

impl Clone for PClient{
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.0;
            rc_impl.interface.add_ref();
        }

        Self(self.0)
    }
}

impl Rc for PClient {
    fn as_base(&self) -> &cef_dll_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.0;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl ImplClient for PClient{
    fn get_raw(&self) -> *mut cef_dll_sys::_cef_client_t {
        self.0.cast()
    }
}
