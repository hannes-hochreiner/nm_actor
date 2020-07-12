use std::process::Command;
use super::exec::Exec;
use std::error::Error;

pub struct CommandExec {}

impl Exec for CommandExec {
    fn exec(&mut self, command: &String, args: &Vec<&String>) -> Result<String, Box<dyn Error>> {
        let output = Command::new(command).args(args).output()?;

        Ok(String::from_utf8(output.stdout)?)
    }
}
