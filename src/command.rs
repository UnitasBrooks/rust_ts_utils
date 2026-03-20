use std::collections::HashMap;
use std::error::Error;
pub trait Command {
    fn help(&self) -> &'static str;
    fn command(&self) -> &'static str;
    fn run(&self, args: &HashMap<String, String>) -> Result<(), Box<dyn Error>>;
}