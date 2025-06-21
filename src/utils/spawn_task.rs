use cef::rc::{Rc, RcImpl};
use cef::{ImplTask, WrapTask};
use cef_dll_sys::cef_task_t;
use std::{cell::RefCell, ptr};

pub struct SpawnWindowTask {
    pub callback: RefCell<Option<Box<dyn FnOnce() + Send>>>,
    object: *mut RcImpl<cef_task_t, Self>,
}

impl SpawnWindowTask {
    pub fn new(callback: impl FnOnce() + Send + 'static) -> Self {
        Self {
            callback: RefCell::new(Some(Box::new(callback))),
            object: ptr::null_mut(),
        }
    }

    pub fn get_raw(&self) -> *mut cef_task_t {
        self.object.cast()
    }
}

impl Clone for SpawnWindowTask {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref(); // manually increment ref count
        }

        Self {
            callback: RefCell::new(None), // cannot clone the callback
            object: self.object,
        }
    }
}

impl ImplTask for SpawnWindowTask {
    fn execute(&self) {
        if let Some(cb) = self.callback.borrow_mut().take() {
            cb();
        }
    }

    fn get_raw(&self) -> *mut cef_task_t {
        self.object.cast()
    }
}

impl WrapTask for SpawnWindowTask {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_task_t, Self>) {
        self.object = object;
    }
}

impl Rc for SpawnWindowTask {
    fn as_base(&self) -> &cef_dll_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}
