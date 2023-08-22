#![allow(dead_code)]
//                                   1  1  1  1  1  1
//     0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
//   +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |                      ID                       |
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |                    QDCOUNT                    |
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |                    ANCOUNT                    |
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |                    NSCOUNT                    |
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |                    ARCOUNT                    |
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

#[derive(Debug, PartialEq)]
struct Header {
    id: u16,
    qr: QueryResponse,
    opcode: Opcode,
    aa: bool,
    tc: bool,
    rd: bool,
    ra: bool,
    z: u8,
    rcode: ResponseCode,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

#[derive(Debug, PartialEq)]
enum QueryResponse {
    Query,
    Response,
}

impl From<bool> for QueryResponse {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Response,
            false => Self::Query,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Opcode {
    Query,               // (0) Standard: we have a name, we want an address
    InverseQuery,        // (1) Inverse: we have an address, we want a name
    ServerStatusRequest, // (2) Status: we want to know if the server is online
    Reserved(u8),        // (3-15) Reserved: reserved for future use
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Query,
            1 => Self::InverseQuery,
            2 => Self::ServerStatusRequest,
            x if (3..16).contains(&x) => Self::Reserved(x),
            _ => panic!("Opcode cannot exceed 15."),
        }
    }
}

#[derive(Debug, PartialEq)]
enum ResponseCode {
    NoError,
    FormatError,    // name server unable to interpret query
    ServerFailure,  // name server unable to process query due to internal error
    NameError,      // if from an authoritative name server, the queried name does not exist
    NotImplemented, // queried name server does not support request
    Refused,        // queried name server refuses request for policy reasons.
    Reserved(u8),   // reserved for future use
}

impl From<u8> for ResponseCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NoError,
            1 => Self::FormatError,
            2 => Self::ServerFailure,
            3 => Self::NameError,
            4 => Self::NotImplemented,
            5 => Self::Refused,
            x if (6..16).contains(&x) => Self::Reserved(x),
            _ => unreachable!(),
        }
    }
}

impl Header {
    fn parse(input: &[u8]) -> Self {
        let id = Self::id(input.get(..2).unwrap().try_into().unwrap());

        let flag_byte_one = *input.get(2).unwrap();

        let qr = Self::qr(flag_byte_one);
        let opcode = Self::opcode(flag_byte_one);
        let aa = Self::aa(flag_byte_one);
        let tc = Self::tc(flag_byte_one);
        let rd = Self::rd(flag_byte_one);

        let flag_byte_two = *input.get(3).unwrap();

        let ra = Self::ra(flag_byte_two);
        let z = Self::z(flag_byte_two);
        let rcode = Self::rcode(flag_byte_two);

        let qdcount = Self::qdcount(input.get(4..6).unwrap().try_into().unwrap());
        let ancount = Self::ancount(input.get(6..8).unwrap().try_into().unwrap());
        let nscount = Self::nscount(input.get(8..10).unwrap().try_into().unwrap());
        let arcount = Self::arcount(input.get(10..12).unwrap().try_into().unwrap());

        Header {
            id,
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            z,
            rcode,
            qdcount,
            ancount,
            nscount,
            arcount,
        }
    }

    fn id(input: &[u8; 2]) -> u16 {
        u16::from_be_bytes(*input)
    }

    fn qr(input: u8) -> QueryResponse {
        ((input & 0b1000_0000) != 0).into()
    }

    fn opcode(input: u8) -> Opcode {
        ((input & 0b0111_1000) >> 3).into()
    }

    fn aa(input: u8) -> bool {
        (input & 0b0000_0100) != 0
    }

    fn tc(input: u8) -> bool {
        (input & 0b0000_0010) != 0
    }

    fn rd(input: u8) -> bool {
        (input & 0b0000_0001) != 0
    }

    fn ra(input: u8) -> bool {
        (input & 0b1000_0000) != 0
    }

    fn z(input: u8) -> u8 {
        (input & 0b0111_0000) >> 4
    }

    fn rcode(input: u8) -> ResponseCode {
        (input & 0b0000_1111).into()
    }

    fn qdcount(input: [u8; 2]) -> u16 {
        u16::from_be_bytes(input)
    }
    fn ancount(input: [u8; 2]) -> u16 {
        u16::from_be_bytes(input)
    }
    fn nscount(input: [u8; 2]) -> u16 {
        u16::from_be_bytes(input)
    }
    fn arcount(input: [u8; 2]) -> u16 {
        u16::from_be_bytes(input)
    }
}
