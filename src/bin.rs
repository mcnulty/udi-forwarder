#![deny(warnings)]

extern crate udi_forwarder;
extern crate log4rs;

use udi_forwarder::monitor_udi_filesystem;

pub fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    monitor_udi_filesystem("/tmp/udi/mcnulty", "127.0.0.1", 8888).unwrap();
}
