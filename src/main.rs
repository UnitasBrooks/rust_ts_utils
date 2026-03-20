use std::collections::HashMap;
use std::env;
use crate::command::Command;
use crate::listen::Listener;
use crate::latency::Latency;
mod listen;
mod latency;
mod command;
mod transport;



fn main() -> Result<(), Box<dyn std::error::Error>>{

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

    // I should probably use a crate here, but this works!
    let mut arg_map: HashMap<String, String> = HashMap::new();
    let mut i = 1;
    while i  < args.len() - 2 {
        println!("{} {}", args[i + 1], args[i + 2]);
        arg_map.insert(args[i + 1].clone(), args[i + 2].clone());
        i += 2;
    }

    // TODO: it would be great to chain commands, IE latency + jitter or just latency or just jitter etc.
    for command in &commands {
        if command.command() == args[1] {
            return command.run(&arg_map);
        }
    }

    return Ok(());
}