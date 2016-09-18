#![deny(warnings)]

mod udi;

extern crate notify;

use std::thread;

use mio::{EventLoop, Handler, Sender}

// TODO need mio Handler that processes both WS messages and
// FS events. This handler will also write to the FS and WS
// connections depending on events. The WS will need to be
// registered with the handler somehow. 

//
// Use notify to monitor the root filesystem
//
fn setup_monitor(root: &str, event_sink: Sender<?>) {

}

fn setup_ws_server(bind_address: &str, port: u16) {

}

// Origin Flow
//
// Start udi-forwarder
// Monitor UDI root dir for new entries
// When UDI root dir has new entry, attempt to perform UDI handshake.
// If successful, send new process event to forwarder
pub fn monitor_udi_filesystem(root: &str, bind_address: &str, port: u16) {

}

pub fn forward_udi_filesystem(root: &str, host_address: &str, port: u16) {

}
