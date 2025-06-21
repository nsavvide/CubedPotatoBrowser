mod browser;
mod handlers;
mod sandbox;
mod utils;

use cef::rc::Rc;
use cef::{args::Args, *};
use handlers::app::PApp;
use std::sync::{Arc, Mutex};

fn main() {
    let _ = api_hash(sys::CEF_API_VERSION_LAST, 0);
    let args = Args::new();
    let cmd = args.as_cmd_line().unwrap();
    let sandbox = sandbox::create_sandbox();
    let is_browser_process = cmd.has_switch(Some(&CefString::from("type"))) != 1;

    let window = Arc::new(Mutex::new(None));
    let mut app = PApp::new(window.clone());

    let ret = execute_process(
        Some(args.as_main_args()),
        Some(&mut app),
        sandbox.as_mut_ptr(),
    );

    if is_browser_process {
        assert!(ret == -1, "cannot execute browser process");
    } else {
        assert!(ret >= 0, "cannot execute non-browser process");
        return;
    }

    let settings = Settings::default();
    assert_eq!(
        initialize(
            Some(args.as_main_args()),
            Some(&settings),
            Some(&mut app),
            sandbox.as_mut_ptr()
        ),
        1
    );

    run_message_loop();

    let window = window.lock().expect("Failed to lock window");
    let window = window.as_ref().expect("Window is None");
    assert!(window.has_one_ref());

    shutdown();
}
