#![deny(warnings)]

mod udi;

extern crate mio;
extern crate ws;
extern crate notify;

use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::channel;

use mio::{EventLoop, Handler};

use notify::{RecommendedWatcher, Watcher};

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
                 event_sink: mio::Sender<EventMessage>) -> 
                 notify::Result<JoinHandle<std::io::Result<()>>> {

    let (tx, rx) = channel();

    let watcher_result: notify::Result<RecommendedWatcher> = Watcher::new(tx);
    watcher_result.and_then(|mut watcher| watcher.watch(root))
        .map(|watcher| {
            thread::spawn(move || {
                loop {
                    match rx.recv() {
                        // TODO handle filesystem events
                        _ => ()
                    }
                }
            })
        })
}

struct LocalWsHandler {
    sender: ws::Sender,
}

impl ws::Handler for LocalWsHandler {

    fn on_message(&mut self, message: ws::Message) -> ws::Result<()> {
        Ok(()) as ws::Result<()>
    }
}

struct LocalWsFactory;

impl ws::Factory for LocalWsFactory {
    type Handler = LocalWsHandler;

    fn connection_made(&mut self, sender: ws::Sender) -> LocalWsHandler  {
        LocalWsHandler { sender: sender }
    }
}

fn setup_ws_server(bind_address: &str,
                   port: u16,
                   event_sink: mio::Sender<EventMessage>) -> ws::Result<ws::Sender> {

    let web_socket_result = ws::Builder::new().build(LocalWsFactory);

    web_socket_result.and_then(|web_socket| web_socket.listen((bind_address, port)))
        .map(|web_socket| web_socket.broadcaster())
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
