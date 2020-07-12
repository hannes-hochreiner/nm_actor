mod mock_exec;
use std::sync::{Arc, Mutex};
use nm_actor::actor::Actor;
use mock_exec::MockExec;

#[test]
fn it_can_check_the_host_success() {
    let list: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let mut act = Actor::new(MockExec{
        exec_results: vec![
            Ok(format!("hostname.domain has address 192.168.0.1"))
        ],
        list: list.clone()
    });

    assert_eq!(act.check_host_reachable(&String::from("hostname"), &String::from("hostname.domain has address")).unwrap(), true);

    let mut list = list.lock().unwrap();

    assert_eq!((*list).remove(0), "exec: command:\"host\" args:[\"-W\", \"1\", \"hostname\"]");
}

#[test]
fn it_can_check_the_host_fail() {
    let list: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let mut act = Actor::new(MockExec{
        exec_results: vec![
            Ok(format!("Host hostname not found: 3(NXDOMAIN)"))
        ],
        list: list.clone()
    });

    assert_eq!(act.check_host_reachable(&String::from("hostname"), &String::from("hostname.domain has address")).unwrap(), false);

    let mut list = list.lock().unwrap();

    assert_eq!((*list).remove(0), "exec: command:\"host\" args:[\"-W\", \"1\", \"hostname\"]");
}

#[test]
fn it_can_activate_the_vpn() {
    let list: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let mut act = Actor::new(MockExec{
        exec_results: vec![
            Ok(format!(""))
        ],
        list: list.clone()
    });

    act.set_vpn_active(&String::from("vpn"), true).unwrap();

    let mut list = list.lock().unwrap();

    assert_eq!((*list).remove(0), "exec: command:\"nmcli\" args:[\"c\", \"up\", \"vpn\"]");
}

#[test]
fn it_can_deactivate_the_vpn() {
    let list: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let mut act = Actor::new(MockExec{
        exec_results: vec![
            Ok(format!(""))
        ],
        list: list.clone()
    });

    act.set_vpn_active(&String::from("vpn"), false).unwrap();

    let mut list = list.lock().unwrap();

    assert_eq!((*list).remove(0), "exec: command:\"nmcli\" args:[\"c\", \"down\", \"vpn\"]");
}
