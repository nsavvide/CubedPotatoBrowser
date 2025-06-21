use std::io::Write;
use std::os::unix::net::UnixStream;

const SOCKET_PATH: &str = "/tmp/potato.sock";

fn main() {
    match UnixStream::connect(SOCKET_PATH) {
        Ok(mut stream) => {
            let msg = "OpenNewWindow\n";
            stream.write_all(msg.as_bytes()).expect("Failed to send message");
            println!("[Client] Sent: {}", msg.trim());
        }
        Err(e) => {
            eprintln!("[Client] Failed to connect to server: {}", e);
        }
    }
}
