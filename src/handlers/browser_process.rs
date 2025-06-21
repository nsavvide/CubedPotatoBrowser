use crate::browser::client::PClient;
use crate::browser::window::create_main_window;
use crate::Window;
use cef::{rc::*, *};
use std::sync::{Arc, Mutex};

pub struct PBrowserProcessHandler {
    pub object: *mut RcImpl<cef_dll_sys::cef_browser_process_handler_t, Self>,
    pub window: Arc<Mutex<Option<Window>>>,
}

impl PBrowserProcessHandler {
    pub fn new(window: Arc<Mutex<Option<Window>>>) -> BrowserProcessHandler {
        BrowserProcessHandler::new(Self {
            object: std::ptr::null_mut(),
            window,
        })
    }
}

impl Rc for PBrowserProcessHandler {
    fn as_base(&self) -> &cef_dll_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl WrapBrowserProcessHandler for PBrowserProcessHandler {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_dll_sys::_cef_browser_process_handler_t, Self>) {
        self.object = object;
    }
}

impl Clone for PBrowserProcessHandler {
    fn clone(&self) -> Self {
        let object = unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
            rc_impl
        };

        let window = self.window.clone();

        Self { object, window }
    }
}

impl ImplBrowserProcessHandler for PBrowserProcessHandler {
    fn get_raw(&self) -> *mut cef_dll_sys::_cef_browser_process_handler_t {
        self.object.cast()
    }

    // The real lifespan of cef starts from `on_context_initialized`, so all the cef objects should be manipulated after that.
    fn on_context_initialized(&self) {
        println!("cef context initialized");

        let browser = Arc::new(Mutex::new(None));
        let mut client = PClient::new(browser.clone());

        let url = CefString::from("https://www.google.com");

        let browser_view = browser_view_create(
            Some(&mut client),
            Some(&url),
            Some(&Default::default()),
            Option::<&mut DictionaryValue>::None,
            Option::<&mut RequestContext>::None,
            Option::<&mut BrowserViewDelegate>::None,
        )
        .expect("Failed to create browser view");

        // Save the browser instance for the keybinding handler
        if let Some(view_browser) = browser_view.browser() {
            *browser.lock().unwrap() = Some(view_browser);
            println!("Browser instance saved in Arc");
        }

        if let Ok(mut window) = self.window.lock() {
            *window = create_main_window(browser_view);
        }
    }
}
