
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

#[derive(Debug)]
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

#[derive(Debug)]
struct DnsError {
    msg: String
}
type DnsResult<T> = Result<T, DnsError>;

// Inspired by TRustDNS encoder/decoder
struct BinDecoder<'a> {
    buffer: &'a [u8],
    index: usize,
}

impl<'a> BinDecoder<'a> {
    
    fn new(buf: &'a [u8]) -> Self {
        BinDecoder{
            buffer: buf,
            index: 0
        }
    }

    fn pop(&mut self) -> DnsResult<u8> {
        self.check_size(1)?;
        let byte = self.buffer[self.index];
        self.index += 1;
        Ok(byte)
    }

    fn check_size(&self, req: usize) -> DnsResult<()> {
        if (self.index + req) > self.buffer.len() {
            Err::<(), DnsError>(DnsError{msg: "End of buffer".to_string()})
        } else {
            Ok(())
        }
    }

    fn read_u16(&mut self) -> DnsResult<u16> {
        let b1 = self.pop()?;
        let b2 = self.pop()?;
        
        Ok(((b1 as u16) << 8) + (b2 as u16))
    }


    fn read_vec(&mut self, len: usize) -> DnsResult<Vec<u8>> {
        self.check_size(len)?;
        let vec :Vec<u8> = self.buffer[self.index..self.index+len].to_vec();
        Ok(vec)
    }


    fn read_char_data(&mut self) -> DnsResult<String> {
        let length :u8 = self.pop()?;

        let char_vec :Vec<u8> = self.read_vec(length as usize)?;

        let data = String::from_utf8(char_vec);

        Ok(data.unwrap())
    }

}

#[derive(Debug)]
struct DnsQuestion<'a> {
    qname: &'a str,
    qtype: u16,
    qclass: u16,
}


impl<'a> DnsQuestion<'a> {

    fn to_bytes(&self) -> Vec<u8> {
        let mut arr :Vec<u8> = vec![0; self.qname.len()+ 6];
        let mut arr_idx = 0;

        for piece in self.qname.split(".") {
            arr[arr_idx] = piece.len() as u8;
            arr_idx += 1;
            arr[arr_idx .. arr_idx + piece.len()].clone_from_slice(piece.as_bytes());
            arr_idx += piece.len();
        }
        // Write null terminating byte
        arr_idx += 1;

        write_u16(self.qtype, &mut arr[arr_idx..arr_idx+2], 0);
        arr_idx += 2;
        write_u16(self.qclass, &mut arr[arr_idx..arr_idx+2], 0);

        return arr
    }
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
    
    fn decode(decoder: &mut BinDecoder) -> DnsResult<DnsHeader> {
        let id = decoder.read_u16()?;
        let qr_op_aa_tc_rd = decoder.pop()?;

        let qr = (0b1000_0000 & qr_op_aa_tc_rd) == 0b1000_0000;
        let op = (0b0_1111_000 & qr_op_aa_tc_rd) >> 3;
        let aa = (0b0000_0100 & qr_op_aa_tc_rd) == 0b0000_0100;
        let tc = (0b0000_0010 & qr_op_aa_tc_rd) == 0b0000_0010;
        let rd = (0b0000_0001 & qr_op_aa_tc_rd) == 0b0000_0001;

        let ra_z_rcode = decoder.pop()?;

        let ra = (0b1000_0000 & ra_z_rcode) == 0b1000_0000;
        let z = (0b0111_0000 & ra_z_rcode) >> 4;
        let rcode = 0b0000_1111 & ra_z_rcode;

        let qdcount = decoder.read_u16()?;
        let ancount = decoder.read_u16()?;
        let nscount = decoder.read_u16()?;
        let arcount = decoder.read_u16()?;

        Ok(DnsHeader{
            id: id,
            qr: qr,
            opcode: op,
            aa: aa,
            tc: tc,
            rd: rd,
            ra: ra,
            z: z,
            rcode: rcode,
            qdcount: qdcount,
            ancount: ancount,
            nscount: nscount,
            arcount: arcount,
        })
    }

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
    let socket = try!(UdpSocket::bind("0.0.0.0:0"));

    let ip = Ipv4Addr::new(8, 8, 8, 8);
    let connection = SocketAddrV4::new(ip, 53);

    let header = DnsHeader{
        id: 1,
        qr: false,
        opcode: 0,
        aa: false,
        tc: false,
        rd: true,
        ra: false,
        z: 0,
        rcode: 0,
        qdcount: 1,
        ancount: 0,
        nscount: 0,
        arcount: 0
    };
    
    let question = DnsQuestion{
        qname: "google.com",
        qtype: 1,
        qclass: 1,
    };
    let mut to_send = [0; 28];

    let head_bytes = header.to_bytes();
    let question_bytes = question.to_bytes();
    
    to_send[..12].clone_from_slice(&head_bytes);
    to_send[12..].clone_from_slice(&question_bytes);

    socket.send_to(&to_send, connection)?;
    println!("Sent! {0}", String::from_utf8_lossy(&to_send));

    // read from the socket
    let mut buf2 = [0; 50];
    let (amt, _) = socket.recv_from(&mut buf2)?;

    println!("Amt is {0}", amt);
    println!("Buf: {0}", String::from_utf8_lossy(&buf2));

    let mut decoder = BinDecoder::new(&buf2);

    let new_header = DnsHeader::decode(&mut decoder);
    match new_header {
        Ok(h) => println!("Header: {:?}", h),
        Err(err) => println!("Decoding err: {}", err.msg),
    }
    
    
Ok(())
} // the socket is closed here
}

fn main() {
	foo().unwrap();
    ()
}



