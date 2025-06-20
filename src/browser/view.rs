use cef::{rc::*, *};

pub struct PWindowDelegate {
    base: *mut RcImpl<cef_dll_sys::_cef_window_delegate_t, Self>,
    pub browser_view: BrowserView,
}

impl PWindowDelegate {
    pub fn new(browser_view: BrowserView) -> WindowDelegate {
        WindowDelegate::new(Self {
            base: std::ptr::null_mut(),
            browser_view,
        })
    }
}

impl WrapWindowDelegate for PWindowDelegate {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_dll_sys::_cef_window_delegate_t, Self>) {
        self.base = object;
    }
}

impl Clone for PWindowDelegate {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.base;
            rc_impl.interface.add_ref();
        }

        Self {
            base: self.base,
            browser_view: self.browser_view.clone(),
        }
    }
}

impl Rc for PWindowDelegate {
    fn as_base(&self) -> &cef_dll_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.base;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl ImplViewDelegate for PWindowDelegate {
    fn on_child_view_changed(
        &self,
        _view: Option<&mut View>,
        _added: ::std::os::raw::c_int,
        _child: Option<&mut View>,
    ) {
        // view.as_panel().map(|x| x.as_window().map(|w| w.close()));
    }

    fn get_raw(&self) -> *mut cef_dll_sys::_cef_view_delegate_t {
        self.base.cast()
    }
}

impl ImplPanelDelegate for PWindowDelegate {}

impl ImplWindowDelegate for PWindowDelegate {
    fn on_window_created(&self, window: Option<&mut Window>) {
        if let Some(window) = window {
            let view = self.browser_view.clone();
            window.add_child_view(Some(&mut (&view).into()));
            window.show();
        }
    }

    fn on_window_destroyed(&self, _window: Option<&mut Window>) {
        quit_message_loop();
    }

    fn with_standard_window_buttons(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
        1
    }

    fn can_resize(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
        1
    }

    fn can_maximize(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
        1
    }

    fn can_minimize(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
        1
    }

    fn can_close(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
        1
    }
}
