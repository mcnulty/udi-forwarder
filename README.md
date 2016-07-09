# UDI Forwarder #

An application, implemented in Rust, to forward a UDI pseudo-filesystem over
a WebSocket.

## Design ##

A UDI forwarder exposes a UDI pseudo-filesystem over a WebSocket by adding a
prologue to UDI request and response messages. The prologue adds a header to
UDI messages that allows the messages to be routed to the correct filesystem
object. The prologue is also used to communicate bootstrap events for
forwarding UDI messages.
 
### Terminology ###

* A _forwarder_ is providing access to an emulated pseudo-filesystem.
* A _origin_ is the host serving the original UDI pseudo-filesystem.

### Forwarder Message Format ###

WebSocket messages exchanged by a forwarder and an origin have the following format:

    +---------------+--------------------------+
    | type (1 byte) | payload (type dependent) |
    +---------------+--------------------------+

Current valid values for type are:

* 0 (UDI)
* 1 (NEW\_PROCESS)

#### Message Type: UDI ####

The primary message type is UDI. The payload is some metadata along with the
content of a UDI message. The payload has the following format:

    +------------------+--------------+-------------+
    | length (4 bytes) | utf-8 string | UDI message |
    +------------------+--------------+-------------+

The string is a UNIX-like path to the pseudo-file that is the target of a
request or the source of a response. The prologue is a relative path, rooted at
the UDI filesystem root configured for the forwarder or origin.

For example, consider the following configuration:

* Forwarder A, UDI root /private/tmp/user/udi
* Origin, UDI root /home/user/.udi

and the components are connected as:

    Consumer <=> Forwarder A <=> Origin

A consumer of forwarder A would send a process request to process 12345 using
/private/tmp/user/udi/12345/request. Forwarder A would send the process request
to the origin with a prologue of /12345/request. The origin would then send
this request to /home/user/.udi/12345/request.

The origin would receive an event for process 12345 from
/home/user/.udi/12345/events and send this event to forwarder A with the
prologue /12345/events.

#### Message Type: NEW\_PROCESS ####

The NEW\_PROCESS message type is to inform a forwarder that a new process has
been created in the pseudo-filesystem and the forwarder should provide access
to the filesystem on its host. The payload has the following format:

    +---------------+
    | pid (4 bytes) |
    +---------------+
