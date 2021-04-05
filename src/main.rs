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
    let host_reachable = actor.check_host_reachable(&host_name, &answer).unwrap_or(false);
    let connection_active = actor.check_connection_active(&*vpn_name)?;

    match (host_reachable, connection_active) {
        (true, false) => {
            info!("host \"{}\" available and connection \"{}\" inactive => bringing vpn up", host_name, vpn_name);
            actor.set_vpn_active(&vpn_name, host_reachable)
        },
        (false, true) => {
            info!("host \"{}\" not available and connection \"{}\" active => bringing vpn down", host_name, vpn_name);
            actor.set_vpn_active(&vpn_name, host_reachable)
        },
        (true, true) => {
            info!("host \"{}\" available and connection \"{}\" is active => no action required", host_name, vpn_name);
            Ok(())
        },
        (false, false) => {
            info!("host \"{}\" not available and connection \"{}\" is not active => no action required", host_name, vpn_name);
            Ok(())
        },
    }
}
