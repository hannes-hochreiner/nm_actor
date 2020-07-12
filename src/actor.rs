pub mod exec;
pub use exec::Exec;
mod command_exec;
use command_exec::CommandExec;
use std::error::Error;

pub struct Actor<T: exec::Exec> {
    exec: T
}

impl Actor<CommandExec> {
    pub fn default() -> Actor<CommandExec> {
        Actor { exec: CommandExec{} }
    }
}

impl<T: exec::Exec> Actor<T> {
    pub fn new(e: T) -> Actor<T> {
        Actor {
            exec: e
        }
    }

    pub fn check_host_reachable(&mut self, host: &String, answer: &String) -> Result<bool, Box<dyn Error>> {
        Ok(self.exec.exec(&format!("host"), &vec![&format!("-W"), &format!("1"), host])?.contains(answer))
    }

    pub fn set_vpn_active(&mut self, vpn: &String, active: bool) -> Result<(), Box<dyn Error>> {
        self.exec.exec(&format!("nmcli"), &vec![&format!("c"), &(match active {
            true => format!("up"),
            false => format!("down")
        }), vpn])?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct ActorError {
    msg: String
}

impl ActorError {
    pub fn new(message: String) -> ActorError {
        ActorError {
            msg: message
        }
    }
}

impl Error for ActorError {}

impl std::fmt::Display for ActorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ActorError: {}", self.msg)
    }
}
