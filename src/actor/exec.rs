use std::error::Error;

pub trait Exec {
    fn exec(&mut self, command: &str, args: &[&str]) -> Result<String, Box<dyn Error>>;
}
