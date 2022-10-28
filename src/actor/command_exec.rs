use std::process::Command;
use super::{ActorError, exec::Exec};
use std::error::Error;

pub struct CommandExec {}

impl Exec for CommandExec {
    fn exec(&mut self, command: &str, args: &[&str]) -> Result<String, Box<dyn Error>> {
        let output = Command::new(command).args(args).output()?;

        if !output.status.success() {
            return Err(Box::new(ActorError::new(String::from_utf8(output.stderr)?)));
        }

        Ok(String::from_utf8(output.stdout)?)
    }
}
