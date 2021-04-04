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
        let res = self.exec.exec(&format!("host"), &vec![&format!("-W"), &format!("1"), host])?;

        debug!("host command returned \"{}\"", res);

        match res.contains(answer) {
            true => {
                debug!("host command return value contained the expected answer");
                Ok(true)
            },
            false => {
                debug!("host command return value did not contain the expected answer");
                Ok(false)
            }
        }
    }

    pub fn set_vpn_active(&mut self, vpn: &String, active: bool) -> Result<(), Box<dyn Error>> {
        self.exec.exec(&format!("nmcli"), &vec![&format!("c"), &(match active {
            true => format!("up"),
            false => format!("down")
        }), vpn])?;

        Ok(())
    }

    pub fn check_connection_active(&mut self, connection: &str) -> Result<bool, Box<dyn Error>> {
        for line in self.exec.exec(&String::from("nmcli"), &vec![&String::from("-t"), &String::from("c")])?.split("\n") {
            match line.split(":").nth(3) {
                Some(s) => {
                    if s == connection {
                        return Ok(true);
                    }
                },
                None => continue
            }
        }

        Ok(false)
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
