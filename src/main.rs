use std::collections::HashMap;
use std::env;
use crate::command::Command;
use crate::listen::Listener;
use crate::latency::Latency;
mod listen;
mod latency;
mod command;

fn main() -> Result<(), String>{

    let commands: Vec<Box<dyn Command>> = vec![
        Box::new(Listener),
        Box::new(Latency)
    ];

    let args: Vec<String> = env::args().collect();

    println!("Program name: {}", args[0]);
    
    if args.len() < 2 || args[1] == "help" {
        for command in &commands {
            println!("{}", command.help());
        }
    }

    let arg_map: HashMap<String, String> = HashMap::new();

    for command in &commands {
        if command.command() == args[1] {
            return command.run(&arg_map);
        }
    }

    return Ok(());
}