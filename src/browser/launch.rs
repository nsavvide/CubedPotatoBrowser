use std::sync::{Arc, Mutex};

use cef::{
    browser_view_create, post_task, CefString, ImplBrowserView, ImplWindow, Task, ThreadId, Window,
};
use cef_dll_sys::cef_thread_id_t;

use crate::browser::{client::PClient, window::create_main_window};
use crate::utils::spawn_task::SpawnWindowTask;

pub fn spawn_browser_window(shared_windows: Arc<Mutex<Vec<Window>>>) {
    let windows = Arc::clone(&shared_windows);

    let inner = SpawnWindowTask::new(move || {
        let browser = Arc::new(Mutex::new(None));
        let mut client = PClient::new(browser.clone());

        let url = CefString::from("https://www.google.com");
        let browser_view = browser_view_create(
            Some(&mut client),
            Some(&url),
            Some(&Default::default()),
            None::<&mut cef::DictionaryValue>,
            None::<&mut cef::RequestContext>,
            None::<&mut cef::BrowserViewDelegate>,
        )
        .expect("Failed to create browser view");

        if let Some(view_browser) = browser_view.browser() {
            *browser.lock().unwrap() = Some(view_browser);
        }

        if let Some(window) = create_main_window(browser_view) {
            window.show();
            windows.lock().unwrap().push(window);
        }
    });

    let mut task = Task::new(inner);

    post_task(ThreadId::from(cef_thread_id_t::TID_UI), Some(&mut task));
}
