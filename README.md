# UDI Forwarder #

An application, implemented in Rust, to forward a UDI pseudo-filesystem over a socket.

## Design ##

A UDI forwarder exposes a UDI pseudo-filesystem over a socket by adding a prologue to UDI request
and response messages. The prologue is a UNIX-like path to the pseudo-file that is the target of
a request or the source of a response.

### Terminology ###

* A _forwarder_ is providing access to an emulated pseudo-filesystem.
* A _origin_ is the host serving the original UDI pseudo-filesystem.

### Prologue Format ###

The prologue is a UNIX-like path to the pseudo-file that is the target of a request or the source
of a response. The prologue is a relative path, rooted at the UDI filesystem root configured for
the forwarder or origin.

For example, consider the following configuration:

* Forwarder A, UDI root /private/tmp/user/udi
* Origin, UDI root /home/user/.udi

and the components are connected as:

    Consumer <=> Forwarder A <=> Origin

A consumer of forwarder A would send a process request to process 12345 using
/private/tmp/user/udi/12345/request. Forwarder A would send the process request to the origin with
a prologue of /12345/request. The origin would then send this request to
/home/user/.udi/12345/request.

The origin would receive an event for process 12345 from /home/user/.udi/12345/events and send this
event to forwarder A with the prologue /12345/events.
