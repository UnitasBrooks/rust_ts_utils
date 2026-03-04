use std::net::UdpSocket;
use std::env;
mod listen;
mod command;
const PORT: u16 = 8080;
const EXPECTED_BYTES: usize = 1328;
const BUFFER_SIZE: usize = 1329; // one more than expected to see if we are getting too much data. 


// I want to do three things here:
// 1. measure latency, so I want to get the time the packet was sent vs. the time we received. 
// 2. measure the receive side jitter, what are the gaps between packets?
// 3. measure the sender side jitter, the packets should be time stamped when sent, what's the jitter that the sender is introducing?

fn main() -> Result<(), String>{
    let addr = format!("localhost:{}", PORT);
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