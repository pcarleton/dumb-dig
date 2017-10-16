# rdns - Rust DNS look up command line tool

How does DNS work?

Port 53.
UDP


Progress:

v0.01
Rust binary sends UDP DNS packet to 8.8.8.8 and receives response

v0
The rust binary listens on a UDP port, and echoes back that it has reversed.  The bash script sends a "Hello" UDP message and prints out its response.

Resources:

[UdpSocket](https://doc.rust-lang.org/std/net/struct.UdpSocket.html)
[UdpSocket source](https://doc.rust-lang.org/src/std/net/udp.rs.html#64)
[`sys_common::net`](https://github.com/rust-lang/rust/blob/master/src/libstd/sys_common/net.rs#L437)

[nslookup commands]( http://www.thegeekstuff.com/2012/07/nslookup-examples/)
interesting: reverse IP lookup, looking what companies use gmail with MX records

[DNS Wikipedia page](https://en.wikipedia.org/wiki/Domain_Name_System#DNS_message_format)

[nslookup man page](http://www.tutorialspoint.com/unix_commands/nslookup.htm)
interesting: there's an interactive mode

[Domain Name RFC1035](https://www.ietf.org/rfc/rfc1035.txt)

