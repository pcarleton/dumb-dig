
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

struct DnsHeader {
    id: u16,

    qr: bool, // single bit
    opcode: u8, // 4 bits
    aa: bool,
    tc: bool, 
    rd: bool,
    
    ra: bool,
    z: u8, // 3 bits
    rcode: u8, // 4 bits

    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}


fn write_u8(a: u8, buf: &mut [u8], idx: usize) -> usize {
    buf[idx] = a;
    return idx+1;
}

fn write_u16(a: u16, buf: &mut [u8], idx: usize ) -> usize {
    buf[idx] = ((a >> 8) & 0xff) as u8;
    buf[idx+1] = (a & 0xff) as u8;
    return idx+2;
}

fn read_u16(buf: &[u8]) -> u16 {
    return ((buf[0] as u16) << 8) | (buf[1] as u16);
}

fn on_bit(a: bool) -> u8 {
    if a {
        return 1
    }
    0
}

impl DnsHeader {
    fn to_bytes(&self) -> [u8; 12] {
        let mut buf :[u8; 12] = [0; 12];
        let mut idx = 0;

        // ID
        idx = write_u16(self.id, &mut buf, idx);

        // QR, AA, TC, RD
        let h1 = 
            on_bit(self.qr) << 7 |
            (self.opcode & 0b1111) << 3 |
            on_bit(self.aa) << 2 |
            on_bit(self.tc) << 1 |
            on_bit(self.rd);

        idx = write_u8(h1, &mut buf, idx);

        // RA, Z, RCODE
        let h2 =
            on_bit(self.ra) << 7 |
            (self.z & 0b111) << 4 |
            (self.rcode & 0b1111);

        idx = write_u8(h2, &mut buf, idx);

        // QDCOUNT
        idx = write_u16(self.qdcount, &mut buf, idx);
        
        // ANCOUNT
        idx = write_u16(self.ancount, &mut buf, idx);

        // NSCOUNT
        idx = write_u16(self.nscount, &mut buf, idx);

        // ARCOUNT
        write_u16(self.arcount, &mut buf, idx);

        return buf;
    }

    //fn from_bytes(bytes: [u8; 12]) -> DnsHeader {
    //    return DnsHeader{
    //        id: 1234
    //    }
    //}
}




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

fn print_bytes(bytes: &[u8]) {
    for b in bytes {
        println!("{:b}", b);
    }
}

fn compare_bytes(a: &[u8], b: &[u8]) {
    println!("A bytes:");
    print_bytes(a);

    println!("B bytes:");
    print_bytes(b);
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
    let header = DnsHeader{
        id: 1,
        qr: false,
        opcode: 0,
        aa: false,
        tc: false,
        rd: false,
        ra: false,
        z: 0,
        rcode: 0,
        qdcount: 1,
        ancount: 0,
        nscount: 0,
        arcount: 0
    };
    let buf = make_packet();

    let mut to_send = [0; 28];

    let head_bytes = header.to_bytes();
    to_send[..12].clone_from_slice(&head_bytes);
    to_send[12..].clone_from_slice(&buf[12..]);

    socket.send_to(&to_send, connection)?;
    println!("Sent! {0}", String::from_utf8_lossy(&to_send));

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



