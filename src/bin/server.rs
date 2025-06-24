use std::{
    io::{BufRead, BufReader},
    os::unix::net::UnixStream,
    sync::{Arc, Mutex},
    thread,
};

use cef::sys;
use cef::{
    api_hash, args::Args, execute_process, initialize, run_message_loop, shutdown, CefString,
    ImplCommandLine, Window,
};
use potato_browser::{browser::launch::spawn_browser_window, handlers::app::PApp, sandbox, utils::adblock::create_adblock_engine};

use std::{fs, os::unix::net::UnixListener, path::Path};

const SOCKET_PATH: &str = "/tmp/potato.sock";

fn start_ipc_thread(windows: Arc<Mutex<Vec<Window>>>, adblock_engine: Arc<Mutex<Engine>>) {
    if Path::new(SOCKET_PATH).exists() {
        fs::remove_file(SOCKET_PATH).expect("Failed to remove old socket");
    }

    let listener = UnixListener::bind(SOCKET_PATH).expect("Failed to bind to socket");
    println!("[Server] Listening on {}", SOCKET_PATH);

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("[Server] New client connected");
                    let windows_clone = Arc::clone(&windows);
                    thread::spawn(move || handle_client(stream, windows_clone, adblock_engine.clone()));
                }
                Err(e) => eprintln!("[Server] Connection failed: {}", e),
            }
        }
    });
}

fn handle_client(stream: UnixStream, windows: Arc<Mutex<Vec<Window>>>, adblock_engine: Arc<Mutex<Engine>>) {
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(message) => {
                println!("[Server] Received: {}", message.trim());
                if message.trim() == "OpenNewWindow" {
                    println!("[Server] Spawning browser window...");
                    spawn_browser_window(windows.clone(), adblock_engine.clone());
                }
            }
            Err(e) => {
                eprintln!("[Server] Error reading message: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let _ = api_hash(sys::CEF_API_VERSION_LAST, 0);
    let args = Args::new();
    let cmd = args.as_cmd_line().unwrap();
    let sandbox = sandbox::create_sandbox();
    let is_browser_process = cmd.has_switch(Some(&CefString::from("type"))) != 1;

    let settings = cef::Settings {
        remote_debugging_port: 2012,
        ..Default::default()
    };

    let windows: Arc<Mutex<Vec<Window>>> = Arc::new(Mutex::new(vec![]));
    let mut app = PApp::new(windows.clone());

    let ret = execute_process(
        Some(args.as_main_args()),
        Some(&mut app),
        sandbox.as_mut_ptr(),
    );
    if is_browser_process {
        assert!(ret == -1, "Cannot execute browser process");
    } else {
        assert!(ret >= 0, "Cannot execute non-browser process");
        return;
    }

    assert_eq!(
        initialize(
            Some(args.as_main_args()),
            Some(&settings),
            Some(&mut app),
            sandbox.as_mut_ptr()
        ),
        1
    );

    let adblock_engine = Arc::new(Mutex::new(create_adblock_engine()));

    start_ipc_thread(windows.clone(), adblock_engine.clone());

    run_message_loop();

    shutdown();
}
