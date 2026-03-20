use std::collections::HashMap;

use crate::command::Command;

pub struct Latency;

// I want to do three things here:
// 1. measure latency, so I want to get the time the packet was sent vs. the time we received. 

// Separate command:
// 2. measure the receive side jitter, what are the gaps between packets?
// 3. measure the sender side jitter, the packets should be time stamped when sent, what's the jitter that the sender is introducing?
impl Command for Latency {
    fn help(&self) -> &'static str {
        return "Measures latency";
    }
    
    fn command(&self) -> &'static str {
        return "latency";
    }

    fn run(&self, args: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", args.len());
        return Ok(());
    }
}