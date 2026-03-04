use std::collections::HashMap;
use std::net::UdpSocket;

use crate::command::Command;

const EXPECTED_BYTES: usize = 1328;
const BUFFER_SIZE: usize = 1329; // one more than expected to see if we are getting too much data. 

pub struct Listener;

impl Command for Listener {
    fn help() -> &'static str {
        return "Listens on a given port and prints bytes received, arguments:\n
         --port <int> (default 8080)";
    }
    
    fn command() -> &'static str {
        return "listen";
    }

    fn run(args: HashMap<String, String>) -> Result<(), String> {
        let user_port_or_default = args
            .get("--port")
            .cloned()
            .unwrap_or("8080".to_string());

        let addr = format!("localhost:{}", user_port_or_default);
        let socket = UdpSocket::bind(&addr).expect("Failed to bind socket");
    
        println!("Listening on {}", addr);
    
        let mut buf= [0u8; BUFFER_SIZE];
    
        loop {
            let (amt, src) = socket.recv_from(&mut buf).expect("Failed to receive data");
        
            println!("Received {} bytes from {}", amt, src);
            if amt != EXPECTED_BYTES {
                return Err(format!("received unexpected byte count {}", amt))
            }
        }
    }
}