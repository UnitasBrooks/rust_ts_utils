use std::{collections::HashMap, error::Error};
use std::net::SocketAddr;
use crate::command::Command;
use crate::transport;


pub struct Listener;

impl Command for Listener {
    fn help(&self) -> &'static str {
        return "Listens on a given port and prints bytes received, arguments:
         --port <int> (default 8080)";
    }
    
    fn command(&self) -> &'static str {
        return "listen";
    }

    fn run(&self, args: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
        let user_port_or_default = args
            .get("--port")
            .cloned()
            .unwrap_or("8080".to_string());


        let function = |a: usize, b: SocketAddr, _: &[u8; 1329]| {
            println!("UDP Packet Size: {}, SRC Addr: {}", a, b);
            return Ok(());
        };

        return transport::execute_packet_fn_on_port(user_port_or_default, function);
    }
}