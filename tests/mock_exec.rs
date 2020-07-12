use std::error::Error;
use std::sync::{Arc, Mutex};
use nm_actor::actor::exec::Exec;

pub struct MockExec {
    pub exec_results: Vec<Result<String, Box<dyn Error>>>,
    pub list: Arc<Mutex<Vec<String>>>
}

impl Exec for MockExec {
    fn exec(&mut self, command: &String, args: &Vec<&String>) -> Result<String, Box<dyn Error>> {
        let mut list = self.list.lock().unwrap();

        (*list).push(format!("exec: command:{:?} args:{:?}", command, args));
        self.exec_results.remove(0)
    }
}
