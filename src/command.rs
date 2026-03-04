use std::collections::HashMap;
pub trait Command {
    fn help() -> &'static str;
    fn command() -> &'static str;
    fn run(args: HashMap<String, String>) -> Result<(), String>;
}