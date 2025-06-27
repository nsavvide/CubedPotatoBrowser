use adblock::Engine;
use cef::{
    browser_view_create, post_task, CefString, ImplBrowserView, ImplView, ImplWindow, Task,
    ThreadId, Window,
};
use cef_dll_sys::cef_thread_id_t;
use std::sync::{Arc, Mutex};

use crate::browser::{client::PClient, window::create_main_window};
use crate::utils::spawn_task::SpawnWindowTask;
use std::sync::atomic::{AtomicI32, Ordering};

static WINDOW_ID_COUNTER: AtomicI32 = AtomicI32::new(1);

pub fn spawn_browser_window(
    shared_windows: Arc<Mutex<Vec<Window>>>,
    adblock_engine: Arc<Mutex<Engine>>,
) {
    let windows = Arc::clone(&shared_windows);
    let engine = Arc::clone(&adblock_engine);

    let inner = SpawnWindowTask::new(move || {
        let browser = Arc::new(Mutex::new(None));
        let window = Arc::new(Mutex::new(None));

        let mut client = PClient::new(
            browser.clone(),
            window.clone(),
            windows.clone(),
            engine.clone(),
        );

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

        if let Some(win) = create_main_window(browser_view) {
            let id = WINDOW_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
            win.set_id(id);
            println!("[spawn_browser_window] Created window with ID: {}", id);

            win.show();
            *window.lock().unwrap() = Some(win.clone());
            windows.lock().unwrap().push(win);
        }
    });

    let mut task = Task::new(inner);
    post_task(ThreadId::from(cef_thread_id_t::TID_UI), Some(&mut task));
}
