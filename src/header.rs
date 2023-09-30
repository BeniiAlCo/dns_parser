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
pub struct Header {
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
    Query,    // (0)
    Response, // (1)
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

// TODO: Make AA into an enum that is either: authoritative, non-authoritative, or not an answer?
// TODO: Make RD into an enum that is either: desired, or undesired?
// TODO: MAke RA into an enum that is either: avaliable, or unavaliable?

#[derive(Debug, PartialEq)]
enum ResponseCode {
    NoError,        // (0)
    FormatError,    // (1) name server unable to interpret query
    ServerFailure,  // (2) name server unable to process query due to internal error
    NameError,      // (3) if from an authoritative name server, the queried name does not exist
    NotImplemented, // (4) queried name server does not support request
    Refused,        // (5) queried name server refuses request for policy reasons.
    Reserved(u8),   // (6-15) reserved for future use
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
    fn parse(input: &[u8]) -> Result<Self, String> {
        if input.len() < 12 {
            Err("input is not long enough to contain a DNS header".into())
        } else {
            let id = Self::id(input.get(..2).unwrap());

            let flag_byte_one = input.get(2).unwrap();

            let qr = Self::qr(flag_byte_one);
            let opcode = Self::opcode(flag_byte_one);
            let aa = Self::aa(flag_byte_one);
            let tc = Self::tc(flag_byte_one);
            let rd = Self::rd(flag_byte_one);

            let flag_byte_two = input.get(3).unwrap();

            let ra = Self::ra(flag_byte_two);
            let z = Self::z(flag_byte_two)?;
            let rcode = Self::rcode(flag_byte_two);

            let qdcount = Self::qdcount(input.get(4..6).unwrap());
            let ancount = Self::ancount(input.get(6..8).unwrap());
            let nscount = Self::nscount(input.get(8..10).unwrap());
            let arcount = Self::arcount(input.get(10..12).unwrap());

            Ok(Header {
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
            })
        }
    }

    fn parse_two_bytes(input: &[u8]) -> u16 {
        u16::from_be_bytes(
            input
                .split_at(std::mem::size_of::<u16>())
                .0
                .try_into()
                .unwrap(),
        )
    }

    pub fn id(input: &[u8]) -> u16 {
        Self::parse_two_bytes(input)
    }

    fn qr(input: &u8) -> QueryResponse {
        ((input & 0b1000_0000) != 0).into()
    }

    fn opcode(input: &u8) -> Opcode {
        ((input & 0b0111_1000) >> 3).into()
    }

    fn aa(input: &u8) -> bool {
        (input & 0b0000_0100) != 0
    }

    fn tc(input: &u8) -> bool {
        (input & 0b0000_0010) != 0
    }

    fn rd(input: &u8) -> bool {
        (input & 0b0000_0001) != 0
    }

    fn ra(input: &u8) -> bool {
        (input & 0b1000_0000) != 0
    }

    fn z(input: &u8) -> Result<u8, String> {
        let z = (input & 0b0111_0000) >> 4;
        if z == 0 {
            Ok(z)
        } else {
            Err(format!("`z` value was not `000`. Instead it was: {z}"))
        }
    }

    fn rcode(input: &u8) -> ResponseCode {
        (input & 0b0000_1111).into()
    }

    fn qdcount(input: &[u8]) -> u16 {
        Self::parse_two_bytes(input)
    }

    fn ancount(input: &[u8]) -> u16 {
        Self::parse_two_bytes(input)
    }

    fn nscount(input: &[u8]) -> u16 {
        Self::parse_two_bytes(input)
    }

    fn arcount(input: &[u8]) -> u16 {
        Self::parse_two_bytes(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn header_cannot_parse_slice_shorter_than_12() {
        proptest! (|(input in [any::<u8>(); 1])| {
            prop_assert!(Header::parse(&input).is_err());
        })
    }

    #[test]
    fn header_parses_correct_length_slices() {
        proptest! (|(input in [any::<u8>(); 12])| {
            if (input[3] & 0b0111_0000) == 0 {
                prop_assert!(Header::parse(&input).is_ok());
            } else {
                prop_assert!(Header::parse(&input).is_err());
            }
        })
    }

    #[test]
    fn id_is_valid_for_all_two_byte_values() {
        proptest! (|(input in [any::<u8>(); 2])| {
            Header::id(&input);
        });
    }

    #[test]
    fn qr_value_correct_for_all_u8_values() {
        proptest! (|(input in any::<u8>())| {
            let qr = Header::qr(&input);
            if (input & 0b1000_0000) == 0 {
                prop_assert!(qr == QueryResponse::Query);
            } else {
                prop_assert!(qr == QueryResponse::Response);
            }
        });
    }

    #[test]
    fn opcode_value_correct_for_all_u8_values() {
        proptest! (|(input in any::<u8>())| {
            let opcode = Header::opcode(&input);
            match (input & 0b0111_1000) >> 3 {
                0 => prop_assert!(opcode == Opcode::Query),
                1 => prop_assert!(opcode == Opcode::InverseQuery),
                2 => prop_assert!(opcode == Opcode::ServerStatusRequest),
                x if (3..=15).contains(&x) => prop_assert!(opcode == Opcode::Reserved(x)),
                _ => prop_assert!(false),
            }
        });
    }

    #[test]
    fn aa_value_correct_for_all_u8_values() {
        proptest! (|(input in any::<u8>())| {
            let aa = Header::aa(&input);
            if (input & 0b0000_0100) == 0 {
                prop_assert!(!aa);
            } else {
                prop_assert!(aa);
            }
        });
    }

    #[test]
    fn tc_value_correct_for_all_u8_values() {
        proptest! (|(input in any::<u8>())| {
            let tc = Header::tc(&input);
            if (input & 0b0000_0010) == 0 {
                prop_assert!(!tc);
            } else {
                prop_assert!(tc);
            }
        });
    }

    #[test]
    fn rd_value_correct_for_all_u8_values() {
        proptest! (|(input in any::<u8>())| {
            let rd = Header::rd(&input);
            if (input & 0b0000_0001) == 0 {
                prop_assert!(!rd);
            } else {
                prop_assert!(rd);
            }
        });
    }

    #[test]
    fn ra_value_correct_for_all_u8_values() {
        proptest! (|(input in any::<u8>())| {
            let ra = Header::ra(&input);
            if (input & 0b1000_0000) == 0 {
                prop_assert!(!ra);
            } else {
                prop_assert!(ra);
            }
        });
    }

    #[test]
    fn z_only_valid_when_zero() {
        proptest! (|(input in any::<u8>())| {
            let z = Header::z(&input);
            if (input & 0b0111_0000) == 0 {
                prop_assert!(z.is_ok());
                prop_assert!(z.unwrap() == 0);
            } else {
                prop_assert!(z.is_err());
            }
        });
    }

    #[test]
    fn rcode_value_correct_for_all_u8_values() {
        proptest! (|(input in any::<u8>())| {
            let rcode = Header::rcode(&input);
            match input & 0b0000_1111 {
                0 => prop_assert!(rcode == ResponseCode::NoError),
                1 => prop_assert!(rcode == ResponseCode::FormatError),
                2 => prop_assert!(rcode == ResponseCode::ServerFailure),
                3 => prop_assert!(rcode == ResponseCode::NameError),
                4 => prop_assert!(rcode == ResponseCode::NotImplemented),
                5 => prop_assert!(rcode == ResponseCode::Refused),
                x if (6..=15).contains(&x) => prop_assert!(rcode == ResponseCode::Reserved(x)),
                _ => prop_assert!(false),
            }
        });
    }

    #[test]
    fn qdcount_is_valid_for_all_two_byte_values() {
        proptest! (|(input in [any::<u8>(); 2])| {
            Header::qdcount(&input);
        });
    }

    #[test]
    fn ancount_is_valid_for_all_two_byte_values() {
        proptest! (|(input in [any::<u8>(); 2])| {
            Header::ancount(&input);
        });
    }

    #[test]
    fn nscount_is_valid_for_all_two_byte_values() {
        proptest! (|(input in [any::<u8>(); 2])| {
            Header::nscount(&input);
        });
    }

    #[test]
    fn arcount_is_valid_for_all_two_byte_values() {
        proptest! (|(input in [any::<u8>(); 2])| {
            Header::arcount(&input);
        });
    }
}
