# rdns - Rust DNS look up command line tool

How does DNS work?

Port 53.
UDP

What does having a trailing "." vs. not having a trailing "." mean?

When I query "google.com", I get an IP that reverse looks up to: http://sfo07s16-in-f78.1e100.net/

When I query "google.com.", I get an IP that resolves to google.com - why is that?

What is an authoritative server?  How does recursion work?


Timeline:

- 1968 IMP (Interface Message Processor) BBN wins ARPA contract
- 1974 TCP (Transmission Control Protocol) (Vint Cerf, Bob Kahn)
- 1982 TCP/IP used on ARPANET
- 1983 DNS (Domain Name System) established
- 1985 symbolics.com the first domain name registered


- 1987 RFC1035 - DNS protocol (P. Mockapetris)

RFC1034:
"Host name to address mappings were maintained by the Network Information Center (NIC) in a single file (HOSTS.TXT) which was FTPed by all hosts (RFC-952, RFC-953)"

What is the NIC? Where was it located?

Other RFC's: 799, 819, 830

882, 883

Goals:
- consistent namespace (w/o network identifiers, addresses, etc.)
- distributed (to handle size & frequency of updates)
- source controls tradeoffs (cost of acquiring, speed of update, accuracy) // So if you provide this service, you decide what terms to provide
- Generally useful (not specific to a particular application) addresses, mail data etc.
- tag data with a type, so different protocols can use it.
- Useful for big mainframes and PC's

What is a "virtual circuit"?


Assumptions:
- Trusted name servers they prefer, accept referrals outside trust set
- Prioritize availability over consistency, and fall back to stale data
- Recursive: server pursues info requested on behalf of client
- Iterative (preferred): server tells client where to ask next for info



Domain Concepts:
- Assumes ascii
- Labels can be 63 bytes long max


The ends in a dot thing:
"When a user needs to type a domain name, the length of each label is
omitted and the labels are separated by dots (".").  Since a complete
domain name ends with the root label, this leads to a printed form which
ends in a dot"


Dot at then end means "absolute" (fully specified). No dot means relative.

Why do I get different answers for "google.com" vs. "google.com."?

Total octets for domain name is 255.


What is NetBIOS? And why is it flat?

You can have hyphens in domain names.

Resource Record (RR) types:
A: host address
CNAME: canonical name of an alias (... what is that?)
HINFO: CPU and OS used by a host (... why?)
MX: Mail exchange details for domain
NS: Authoritative name server for the domain
PTR: A pointer to another part of the name space
SOA: identifies the start of a zone of authority (.. what?)

class:
IN: Internet
CH: Chaos (wtf?)

TTL: 32 bit integer in units of seconds

RDATA..
A: IN -> 32 bit IP address, CH (?!) -> domain name w/ 16 bit octal Chaos address
CNAME: a domain name (oh so it just lists another domain name, how is that different from PTR?)
MX: Preference value (16 bit) w/ host name for the owner domain for mail
NS: a host name (is this different than a domain name?)
PTR: a domain name (how is this different from a CNAME?)
SOA: (several fields... what is this?)


(Could reduce DNS TTL's prior to termination, and increase again after termination succeeds... cache invalidation is hard)



CNAME vs. PTR example (still not clear):


For example, suppose a name server was processing a query with for USC-
ISIC.ARPA, asking for type A information, and had the following resource
records:

    USC-ISIC.ARPA   IN      CNAME   C.ISI.EDU

        C.ISI.EDU       IN      A       10.0.0.52

        Both of these RRs would be returned in the response to the type A query,
        while a type CNAME or * query should return just the CNAME.

        Domain names in RRs which point at another name should always point at
        the primary name and not the alias.  This avoids extra indirections in
        accessing information.  For example, the address to name RR for the
        above host should be:

            52.0.0.10.IN-ADDR.ARPA  IN      PTR     C.ISI.EDU

            rather than pointing at USC-ISIC.ARPA.


PTR records seem to be specifically for reverse IP look up.

Interestingly, "Inverse Queries" are for SOA > domain name lookup, but they should not be used the other way around.

(I'm curious if when I query for "x.x.x.x" on nslookup, if it is turning it into a z.y.x.in-addr.arpa query.  Maybe I should write a DNS-dump tool!)

Zones:

"Top node" of a zone - what does that mean?


Secondary servers request AFXR requests to transfer whole zone files to them over the DNS protocol. It should go over TCP it says.



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

