#![deny(warnings)]

mod udi;

extern crate mio;
extern crate ws;
extern crate notify;

use std::result::Result;
use std::error::Error;
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
                 Result<JoinHandle<Result<(), std::io::Error>>, Box<Error>> {

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx));

    try!(watcher.watch(root));

    let join_handle = thread::spawn(move || {
        loop {
            match rx.recv() {
                _ => {
                    return Ok(())
                }
            }
        }
    });

    Ok(join_handle)
}

struct LocalWsHandler {
    sender: ws::Sender,
}

impl ws::Handler for LocalWsHandler {

    fn on_message(&mut self, message: ws::Message) -> ws::Result<()> {
        // TODO
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
                   event_sink: mio::Sender<EventMessage>) -> Result<ws::Sender, Box<Error>> {

    let web_socket = try!(ws::Builder::new().build(LocalWsFactory));

    let sender = web_socket.broadcaster();

    try!(web_socket.listen((bind_address, port)));

    Ok(sender)
}

// Origin Flow
//
// Start udi-forwarder
// Monitor UDI root dir for new entries
// When UDI root dir has new entry, attempt to perform UDI handshake.
// If successful, send new process event to forwarder
pub fn monitor_udi_filesystem(root: &str, bind_address: &str, port: u16) -> Result<(), Box<Error>> {

    let mut event_loop = try!(EventLoop::new());

    let ws_sender = try!(setup_ws_server(bind_address, port, event_loop.channel()));

    try!(setup_monitor(root, event_loop.channel()));

    Ok(try!(event_loop.run(&mut EventHandler{ sender: ws_sender })))
}

pub fn forward_udi_filesystem(root: &str, host_address: &str, port: u16) {

}
