#![deny(warnings)]

extern crate udi_forwarder;

use udi_forwarder::monitor_udi_filesystem;

pub fn main() {
    monitor_udi_filesystem("/tmp", "127.0.0.1", 8888);
}
