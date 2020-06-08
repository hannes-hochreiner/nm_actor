#!/bin/bash
RUST_LOG=info NM_ACTOR_VPN=vpn NM_ACTOR_HOST=host NM_ACTOR_ANSWER="host.domain.net has address 192.168.0.1" /opt/user/cargo/bin/nm_actor "$@" >> /tmp/nm_actor_log.txt 2>&1
