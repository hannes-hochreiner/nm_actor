#[macro_use] extern crate log;
use std::env;
use std::error::Error;
use nm_actor::actor::Actor;

fn main() -> Result<(), Box<dyn Error>> {
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

    let mut actor = Actor::default();
    let res = actor.check_host_reachable(host_name, answer)?;

    match res {
        true => info!("host available => bringing vpn up"),
        false => info!("host not available => bringing vpn down")
    }

    actor.set_vpn_active(vpn_name, res)
}
