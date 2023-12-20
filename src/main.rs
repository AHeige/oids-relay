use std::net::TcpListener;
use tungstenite::accept;

/// A WebSocket echo server
fn main () {
    println!("Starting ws server");
    let server = TcpListener::bind("127.0.0.1:9001").expect("Could not bind to adr");
    println!("Waiting for connections...");
    for stream in server.incoming() {
        println!("Spawning new thread...");
        std::thread::spawn(move || {
            let mut websocket = accept(stream.expect("Stream err")).expect("Could not accept new connections");
            loop {
                if websocket.can_read() {
                    let msg = websocket.read().expect("Could not read");
                    if msg.is_binary() || msg.is_text() {
                        println!("{}", msg);
                        websocket.send(msg).expect("Could not send!")
                    }
                } else {
                    println!("Connection closed");
                    break;
                }
            }
        });
        
    }

}
