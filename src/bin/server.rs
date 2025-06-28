use std::{
    fs,
    io::{BufRead, BufReader},
    os::unix::net::{UnixListener, UnixStream},
    path::Path,
    sync::{Arc, Mutex},
    thread,
};

use adblock::Engine;
use cef::sys;
use cef::{
    api_hash, args::Args, execute_process, initialize, run_message_loop, shutdown, CefString,
    ImplCommandLine, Window,
};
use log::{debug, error, info, warn};
use potato_browser::{
    browser::launch::spawn_browser_window, handlers::app::PApp, sandbox,
    utils::adblock::create_adblock_engine,
};

/// Socket path for IPC communication between client and server
const SOCKET_PATH: &str = "/tmp/potato.sock";

/// Starts the IPC thread to handle client connections
fn start_ipc_thread(
    windows: Arc<Mutex<Vec<Window>>>,
    adblock_engine: Arc<Mutex<Engine>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Clean up any existing socket
    if Path::new(SOCKET_PATH).exists() {
        if let Err(e) = fs::remove_file(SOCKET_PATH) {
            error!("Failed to remove old socket: {}", e);
            return Err(Box::new(e));
        }
    }

    let listener = match UnixListener::bind(SOCKET_PATH) {
        Ok(listener) => listener,
        Err(e) => {
            error!("Failed to bind to socket {}: {}", SOCKET_PATH, e);
            return Err(Box::new(e));
        }
    };

    info!("Server listening on {}", SOCKET_PATH);

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    info!("New client connected");
                    let windows_clone = Arc::clone(&windows);
                    let adblock_engine_clone = Arc::clone(&adblock_engine);
                    thread::spawn(move || {
                        handle_client(stream, windows_clone, adblock_engine_clone)
                    });
                }
                Err(e) => error!("Connection failed: {}", e),
            }
        }
    });

    Ok(())
}

/// Handles client connections and processes IPC messages
fn handle_client(
    stream: UnixStream,
    windows: Arc<Mutex<Vec<Window>>>,
    adblock_engine: Arc<Mutex<Engine>>,
) {
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(message) => {
                let trimmed_message = message.trim();
                debug!("Received IPC message: {}", trimmed_message);

                match trimmed_message {
                    "OpenNewWindow" => {
                        info!("Opening new browser window");
                        spawn_browser_window(windows.clone(), adblock_engine.clone());
                    }
                    "Shutdown" => {
                        info!("Received shutdown signal");
                        break;
                    }
                    _ => {
                        warn!("Unknown IPC message: {}", trimmed_message);
                    }
                }
            }
            Err(e) => {
                error!("Error reading IPC message: {}", e);
                break;
            }
        }
    }

    debug!("Client connection closed");
}

fn main() {
    // Initialize logging
    env_logger::init();

    let _ = api_hash(sys::CEF_API_VERSION_LAST, 0);
    let args = Args::new();

    let cmd = match args.as_cmd_line() {
        Some(cmd) => cmd,
        None => {
            error!("Failed to create command line");
            return;
        }
    };

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
        if ret != -1 {
            error!("Cannot execute browser process, returned: {}", ret);
            return;
        }
    } else {
        if ret < 0 {
            error!("Cannot execute non-browser process, returned: {}", ret);
            return;
        }
        return;
    }

    let init_result = initialize(
        Some(args.as_main_args()),
        Some(&settings),
        Some(&mut app),
        sandbox.as_mut_ptr(),
    );

    if init_result != 1 {
        error!("CEF initialization failed with code: {}", init_result);
        return;
    }

    info!("CEF initialized successfully");

    let adblock_engine = create_adblock_engine();
    info!("Adblock engine created");

    if let Err(e) = start_ipc_thread(windows.clone(), adblock_engine.clone()) {
        error!("Failed to start IPC thread: {}", e);
        shutdown();
        return;
    }

    info!("Starting message loop");
    run_message_loop();

    info!("Shutting down");
    shutdown();
}
