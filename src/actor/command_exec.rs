use std::process::Command;
use super::{ActorError, exec::Exec};
use std::error::Error;

pub struct CommandExec {}

impl Exec for CommandExec {
    fn exec(&mut self, command: &String, args: &Vec<&String>) -> Result<String, Box<dyn Error>> {
        let output = Command::new(command).args(args).output()?;

        if !output.status.success() {
            return Err(Box::new(ActorError::new(String::from("command failed"))));
        }

        Ok(String::from_utf8(output.stdout)?)
    }
}
