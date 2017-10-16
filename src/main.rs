
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};


fn make_packet() -> [u8; 28] {
	//	0x0000:  f4f2 6d87 af3c 040c cee2 507e 0800 4500  ..m..<....P~..E.
	//  0x0010:  0038 412e 0000 4011 1235 c0a8 569a 0808  .8A...@..5..V...
	//  0x0020:  0808 857d 0035 0024 f56a 497d 0100 0001  ...}.5.$.jI}....
	//  0x0030:  0000 0000 0000 0667 6f6f 676c 6503 636f  .......google.co
	//  0x0040:  6d00 0001 0001  

    return [
        0u8,
        1u8, // ID Packet

        0u8, // QD = 0 query, OPCODE = 0 standard query
        // AA = 0 only for responses
        // TC = 0 truncation in responses
        // RD = ? recursion desired
        // RA = 0 recursion available

        0u8, // Z = 0 (future use?), RCODE = 0 response code
        0u8, 1u8, // QDCOUNT = 1
        0u8, 0u8, // ANCOUNT = 0
        0u8, 0u8, // NSCOUNT = 0
        0u8, 0u8, // ARCOUNT = 0

        // QNAME

        6u8, // 6
        103u8, // g
        111u8, // o
        111u8, // o
        103u8, // g
        108u8, // l
        101u8, // e

        3u8, // 3
        99u8, // c
        111u8, // o
        109u8, // m
		0u8, // \0



        
        // QTYPE
        0u8,
        1u8, // A type
        
        // QCLASS
        0u8,  // I
        1u8, // N (for Internet)
    ]
}

fn foo() -> std::io::Result<()> {
{
    //let mut socket = UdpSocket::bind("127.0.0.1:34254")?;
    let ip = Ipv4Addr::new(8, 8, 8, 8);
    let connection = SocketAddrV4::new(ip, 53);
    let socket = try!(UdpSocket::bind("0.0.0.0:0"));

    // read from the socket
    //let mut buf = [0; 10];
    //let (amt, src) = socket.recv_from(&mut buf)?;

    // send a reply to the socket we received data from
    let buf = make_packet();
    socket.send_to(&buf, connection)?;
    println!("Sent! {0}", String::from_utf8_lossy(&buf));

    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create("dnsquery.txt")?;
    file.write_all(&buf)?;
    
    // read from the socket
    let mut buf2 = [0; 50];
    let (amt, src) = socket.recv_from(&mut buf2)?;

    println!("Amt is {0}", amt);
    println!("Buf: {0}", String::from_utf8_lossy(&buf2));
Ok(())
} // the socket is closed here
}

fn main() {
	foo().unwrap();
    ()
}



