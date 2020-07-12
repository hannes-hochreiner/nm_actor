.. image:: https://github.com/hannes-hochreiner/nm_actor/workflows/Rust/badge.svg
  :alt: Rust

NM_ACTOR
========

"nm_actor" is a tool to automatically activate and deactivate a VPN connection depending on the network environment.
The tool is meant to be used in conjunction with the NetworkManager's "dispatch" functionality.

The network environment is checked by executing the command "host" with the host provided in the environment variable "NM_ACTOR_HOST".
Afterwards, it is checked whether the output of the command contains the expected answer as provided in the environment variable "NM_ACTOR_ANSWER".

Environment Variables
---------------------

:RUST_LOG: The log level for the Rust "log" module.
:NM_ACTOR_VPN: Id of the VPN connection to manage.
:NM_ACTOR_HOST: Host to be checked.
:NM_ACTOR_ANSWER: Expected answer.

Deployment
----------

"nm_actor" can be installed using cargo.

.. code-block:: shell

  cargo install --git https://github.com/hannes-hochreiner/nm_actor

A shell script can then be created in "/etc/NetworkManager/dispatch.d" to call "nm_actor".
