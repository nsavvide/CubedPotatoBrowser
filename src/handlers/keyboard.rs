use crate::browser::client::PClient;
use crate::browser::keybinds::{KeybindingManager, VimAction};
use crate::utils::string::string_utf16_to_utf8;
use cef::rc::{Rc, RcImpl};
use cef::{Browser, BrowserSettings, ImplBrowser, ImplBrowserHost, KeyEventType, WindowInfo};
use cef::{CefString, ImplFrame};
use cef::{ImplKeyboardHandler, WrapKeyboardHandler};
use cef_dll_sys::_XEvent;
use cef_dll_sys::cef_key_event_type_t;
use std::sync::{Arc, Mutex};

pub struct PKeyboardHandler {
    pub object: *mut RcImpl<cef_dll_sys::_cef_keyboard_handler_t, Self>,
    pub keybindings: Arc<Mutex<KeybindingManager>>,
    pub browser: Arc<Mutex<Option<cef::Browser>>>,
}

impl PKeyboardHandler {
    pub fn new(browser: Arc<Mutex<Option<Browser>>>) -> Self {
        Self {
            object: std::ptr::null_mut(),
            keybindings: Arc::new(Mutex::new(KeybindingManager::new())),
            browser,
        }
    }
}

impl Rc for PKeyboardHandler {
    fn as_base(&self) -> &cef_dll_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl WrapKeyboardHandler for PKeyboardHandler {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_dll_sys::_cef_keyboard_handler_t, Self>) {
        self.object = object;
    }
}

impl ImplKeyboardHandler for PKeyboardHandler {
    fn get_raw(&self) -> *mut cef_dll_sys::_cef_keyboard_handler_t {
        self.object.cast()
    }

    fn on_key_event(
        &self,
        _browser: Option<&mut cef::Browser>,
        event: Option<&cef::KeyEvent>,
        _os_event: Option<&mut _XEvent>,
    ) -> i32 {
        if let Some(event) = event {
            if event.type_ != KeyEventType::from(cef_key_event_type_t::KEYEVENT_RAWKEYDOWN) {
                return 0;
            }

            let key_str = if event.character == 0 {
                // Ignore non-character key events
                return 0;
            } else {
                match std::char::from_u32(event.character as u32) {
                    Some(ch) => ch.to_string(),
                    None => return 0, // ignore unconvertible chars
                }
            };
            let mut manager = self.keybindings.lock().unwrap();
            println!("Key pressed: {}", key_str);

            if let Some(action) = manager.push_key(&key_str) {
                match action {
                    VimAction::YankUrl => {
                        if let Some(browser) = self.browser.lock().unwrap().as_ref() {
                            if let Some(frame) = browser.main_frame() {
                                let url = frame.url();
                                println!("Current URL: {}", string_utf16_to_utf8(&url));
                            }
                        }
                    }
                    VimAction::OpenDevTools => {
                        println!("Opening DevTools");
                        if let Some(browser) = self.browser.lock().unwrap().as_ref() {
                            let host = browser.host().unwrap();

                            let window_info = None::<&WindowInfo>;
                            let dummy_client = None::<&mut PClient>;
                            let browser_settings = BrowserSettings::default();
                            let settings = Some(&browser_settings);

                            host.show_dev_tools(window_info, dummy_client, settings, None);
                        }
                    }
                    VimAction::GoToBottom => {
                        println!("Scrolling to bottom");
                        if let Some(browser) = self.browser.lock().unwrap().as_ref() {
                            if let Some(frame) = browser.main_frame() {
                                // Scroll to the bottom by executing JS on the main frame:
                                let js_code = CefString::from(
                                    "window.scrollTo(0, document.body.scrollHeight);",
                                );
                                frame.execute_java_script(Some(&js_code), None, 0);
                            }
                        }
                    }
                    VimAction::GoToTop => {
                        println!("Scrolling to top");
                        if let Some(browser) = self.browser.lock().unwrap().as_ref() {
                            if let Some(frame) = browser.main_frame() {
                                // Scroll to the top by executing JS on the main frame:
                                let js_code = CefString::from("window.scrollTo(0, 0);");
                                frame.execute_java_script(Some(&js_code), None, 0);
                            }
                        }
                    }

                    VimAction::ScrollUp => {
                        println!("Scrolling up");
                        if let Some(browser) = self.browser.lock().unwrap().as_ref() {
                            if let Some(frame) = browser.main_frame() {
                                let js_code = CefString::from("window.scrollBy(0, -100);");
                                frame.execute_java_script(Some(&js_code), None, 0);
                            }
                        }
                    }

                    VimAction::ScrollDown => {
                        println!("Scrolling down");
                        if let Some(browser) = self.browser.lock().unwrap().as_ref() {
                            if let Some(frame) = browser.main_frame() {
                                let js_code = CefString::from("window.scrollBy(0, 100);");
                                frame.execute_java_script(Some(&js_code), None, 0);
                            }
                        }
                    }

                    VimAction::ScrollLeft => {
                        println!("Scrolling left");
                        if let Some(browser) = self.browser.lock().unwrap().as_ref() {
                            if let Some(frame) = browser.main_frame() {
                                let js_code = CefString::from("window.scrollBy(-100, 0);");
                                frame.execute_java_script(Some(&js_code), None, 0);
                            }
                        }
                    }

                    VimAction::ScrollRight => {
                        println!("Scrolling right");
                        if let Some(browser) = self.browser.lock().unwrap().as_ref() {
                            if let Some(frame) = browser.main_frame() {
                                let js_code = CefString::from("window.scrollBy(100, 0);");
                                frame.execute_java_script(Some(&js_code), None, 0);
                            }
                        }
                    }

                    VimAction::ScrollDownPage => {
                        println!("Page down");
                        if let Some(browser) = self.browser.lock().unwrap().as_ref() {
                            if let Some(frame) = browser.main_frame() {
                                // Scroll down one "page" (e.g. 80% of the viewport height)
                                let js_code = CefString::from(
                                    "window.scrollBy(0, window.innerHeight * 0.8);",
                                );
                                frame.execute_java_script(Some(&js_code), None, 0);
                            }
                        }
                    }

                    VimAction::ScrollUpPage => {
                        println!("Page up");
                        if let Some(browser) = self.browser.lock().unwrap().as_ref() {
                            if let Some(frame) = browser.main_frame() {
                                let js_code = CefString::from(
                                    "window.scrollBy(0, -window.innerHeight * 0.8);",
                                );
                                frame.execute_java_script(Some(&js_code), None, 0);
                            }
                        }
                    }

                    VimAction::GoToPrevious => {
                        println!("Going back");
                        if let Some(browser) = self.browser.lock().unwrap().as_ref() {
                            if browser.can_go_back() == 1 {
                                browser.go_back();
                            }
                        }
                    }

                    VimAction::GoToNext => {
                        println!("Going forward");
                        if let Some(browser) = self.browser.lock().unwrap().as_ref() {
                            if browser.can_go_forward() == 1 {
                                browser.go_forward();
                            }
                        }
                    }
                    other => println!("VimAction triggered: {:?}", other),
                }

                return 1; // key was handled
            }
        }

        0
    }
}

impl Clone for PKeyboardHandler {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
        }

        Self {
            object: self.object,
            keybindings: self.keybindings.clone(),
            browser: self.browser.clone(),
        }
    }
}
