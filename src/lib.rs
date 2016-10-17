#![deny(warnings)]

mod udi;

extern crate mio;
extern crate ws;
extern crate notify;

#[macro_use]
extern crate log;

use std::error::Error;
use std::path::PathBuf;
use std::result::Result;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::JoinHandle;

use mio::{EventLoop, Handler};

use notify::{RecommendedWatcher, Watcher};

enum EventMessage
{
    WebSocketMessage(ws::Message),
    FilesystemEvent{ path: PathBuf, op: notify::Op},
    FilesystemMonitorTerminated
}

struct EventHandler
{
    sender: ws::Sender,
}

impl Handler for EventHandler {
    type Timeout = ();
    type Message = EventMessage;

    fn notify(&mut self, event_loop: &mut EventLoop<Self>, msg: Self::Message) {

        match msg {
            EventMessage::FilesystemEvent{ path, op } => {
                match self.sender.send(format!("FS Event[{:?}, {:?}]",
                                         path,
                                         op)) {
                    Ok(_) => {},
                    Err(e) => {
                        error!("Failed to send WebSocket message: {}", e);
                        event_loop.shutdown();
                    }
                }
            },
            EventMessage::FilesystemMonitorTerminated => {
                match self.sender.send("FS Monitor Terminated") {
                    Ok(_) => {},
                    Err(e) => {
                        error!("Failed to send WebSocket message: {}", e);
                        event_loop.shutdown();
                    }
                }
            },
            EventMessage::WebSocketMessage(message) => {
                debug!("Received WebSocket message: {}", message);
            }
        }
    }
}


fn run_fs_watcher(root: String,
                  event_sink: &mio::Sender<EventMessage>) -> Result<(), Box<Error>> {

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx));

    try!(watcher.watch(root));

    loop {
        match rx.recv() {
            Ok(notify::Event{ path: Some(path), op: Ok(op) }) => {
                debug!("FilesystemEvent[ path: {:?}, op: {:?} ]",
                       path,
                       op);
                match event_sink.send(EventMessage::FilesystemEvent{ path: path, op: op }) {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Failed to send notify event message: {}", e);
                    }
                }
            },
            Ok(event) => {
                error!("Unexpected event received: {:?}", event);
            },
            Err(err) => {
                return Err(From::from(err))
            }
        }
    }
}

//
// Monitor the root filesystem for new processes
//
fn start_fs_monitor(root: &str,
                    event_sink: mio::Sender<EventMessage>) ->
    JoinHandle<Result<(), String>> {

    let root = root.to_owned();

    thread::spawn(move || {
        match run_fs_watcher(root, &event_sink) {
            Ok(_) => {
                return Ok(())
            },
            Err(e) => {
                debug!("Filesystem monitor terminating: {}", e);
                match event_sink.send(EventMessage::FilesystemMonitorTerminated) {
                    Err(send_error) => {
                        error!("Failed to send filesystem monitor termination message: {}",
                               send_error);
                    },
                    _ => {
                    }
                }
                return Err(From::from((*e).description().to_string()))
            }
        }
    })
}

struct LocalWsHandler {
    event_sink: mio::Sender<EventMessage>
}

impl ws::Handler for LocalWsHandler {

    fn on_message(&mut self, message: ws::Message) -> ws::Result<()> {
        match self.event_sink.send(EventMessage::WebSocketMessage(message)) {
            Err(e) => {
                error!("Failed to send WebSocketMessage to event loop: {}", e);
            },
            _ => {}
        }
        Ok(())
    }
}

struct LocalWsFactory
{
    event_sink: mio::Sender<EventMessage>
}

impl ws::Factory for LocalWsFactory {
    type Handler = LocalWsHandler;

    fn connection_made(&mut self, _: ws::Sender) -> LocalWsHandler  {
        LocalWsHandler { event_sink: self.event_sink.clone() }
    }
}

fn setup_ws_server(bind_address: &str,
                   port: u16,
                   event_sink: mio::Sender<EventMessage>) ->
    Result<(ws::Sender, JoinHandle<Result<(), String>>), Box<Error>> {

    let factory = LocalWsFactory{ event_sink: event_sink.clone() };

    let web_socket = try!(ws::Builder::new().build(factory));

    let sender = web_socket.broadcaster();

    let bind_address = bind_address.to_owned();

    let join_handle = thread::spawn(move || {
        match web_socket.listen((&*bind_address, port)) {
            Err(e) => {
                Err(From::from(e.description().to_string()))
            },
            _ => {
                Err(From::from("WebSocket listen loop returned early"))
            }
        }
    });

    Ok((sender, join_handle))
}

// Origin Flow
//
// Start udi-forwarder
// Monitor UDI root dir for new entries
// When UDI root dir has new entry, attempt to perform UDI handshake.
// If successful, send new process event to forwarder
pub fn monitor_udi_filesystem(root: &str, bind_address: &str, port: u16) -> Result<(), Box<Error>> {

    let mut event_loop = try!(EventLoop::new());

    let (ws_sender, ws_join_handle) = try!(setup_ws_server(bind_address, port, event_loop.channel()));

    let fs_join_handle = start_fs_monitor(root, event_loop.channel());

    match event_loop.run(&mut EventHandler{ sender: ws_sender }) {
        Ok(_) => {
        },
        Err(e) => {
            error!("Event loop failed: {}", e);
        }
    }

    match fs_join_handle.join() {
        Ok(result) => {
            match result {
                Ok(_) => {},
                Err(e) => {
                    error!("Filesystem monitor failed: {}", e);
                }
            }
        },
        Err(e) => {
            panic!(e);
        }
    }

    match ws_join_handle.join() {
        Ok(result) => {
            match result {
                Ok(_) => {},
                Err(e) => {
                    error!("WebSocket listen thread failed: {}", e);
                }
            }
        },
        Err(e) => {
            panic!(e);
        }
    }

    Ok(())
}

// pub fn forward_udi_filesystem(root: &str, host_address: &str, port: u16) {
//
// }
