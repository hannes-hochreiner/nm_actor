#[macro_use] extern crate log;
use std::process::Command;
use std::env;

fn main() {
    env_logger::init();
    let vpn_name = env::var("NM_ACTOR_VPN").expect("no vpn name (\"NM_ACTOR_VPN\") given");
    let host_name = env::var("NM_ACTOR_HOST").expect("no host name (\"NM_ACTOR_HOST\") given");
    let answer = env::var("NM_ACTOR_ANSWER").expect("no expected host answer (\"NM_ACTOR_ANSWER\") given");
    let args: Vec<String> = env::args().collect();

    debug!("{:?}", args);

    if args[1] == "" {
        debug!("no device given => call ignored");
        std::process::exit(0);
    } else if args[1] == vpn_name {
        debug!("device is \"{}\" => call ignored", vpn_name);
        std::process::exit(0);
    }

    let output = Command::new("host")
        .arg("-W")
        .arg("1")
        .arg(host_name)
        .output()
        .expect("failed to execute process");
    let cmd;

    if String::from_utf8(output.stdout).unwrap().contains(&answer) {
        info!("home network available => bringing vpn up");
        cmd = "up";
    } else {
        info!("home network not available => bringing vpn down");
        cmd = "down";
    }

    Command::new("nmcli")
        .arg("c")
        .arg(cmd)
        .arg(vpn_name)
        .output()
        .expect("failed to execute process");
}
