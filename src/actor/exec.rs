use std::error::Error;

pub trait Exec {
    fn exec(&mut self, command: String, args: Vec<String>) -> Result<String, Box<dyn Error>>;
}
