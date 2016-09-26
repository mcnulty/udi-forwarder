#![deny(warnings)]

mod udi;

extern crate mio;
extern crate ws;
extern crate notify;

use std::thread;
use std::sync::mpsc::channel;

use mio::{EventLoop, Handler};

use notify::Watcher;

struct EventMessage;

struct EventHandler
{
    sender: ws::Sender,
}

impl Handler for EventHandler {
    type Timeout = ();
    type Message = EventMessage;

    fn notify(&mut self, event_loop: &mut EventLoop<Self>, msg: Self::Message) {
        // TODO
    }
}

//
// Use notify to monitor the root filesystem
//
fn setup_monitor(root: &str,
                 event_sink: mio::Sender<EventMessage>) -> notify::Result<JoinHandle> {

    let (tx, rx) = channel();

    Watcher::new(tx).and_then(|watcher| watcher.watch(root))
                    .map(|watcher| {
                        thread::spawn(move || {
                            loop {
                                match rx.recv() {
                                    // TODO handle filesystem events
                                    _ => ()
                                }
                            }
                        });
                    });
}

fn setup_ws_server(bind_address: &str,
                   port: u16,
                   event_sink: mio::Sender<EventMessage>) -> ws::Result<ws::Sender> {

    let ws_handler = |out| {
        move |msg| {
            // TODO send message to event_sink
        }
    };

    ws::Builder::new().build(ws_handler)
                      .and_then(|web_socket| web_socket.listen((bind_address, port)))
                      .map(|web_socket| web_socket.broadcaster());
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
