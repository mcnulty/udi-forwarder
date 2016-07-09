#![deny(warnings)]

mod udi;

extern crate notify;

// Origin Flow
//
// Start udi-forwarder
// Monitor UDI root dir for new entries
// When UDI root dir has new entry, attempt to
// perform UDI handshake.
// If successful, send new process event to forwarder

pub fn monitor_udi_filesystem(root: &str, bind_address: &str, port: u16) {

}

pub fn forward_udi_filesystem(root: &str, host_address: &str, port: u16) {

}
