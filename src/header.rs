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

use nom::{
    bits::complete::{bool, take},
    combinator::verify,
    number::complete::be_u16,
    IResult,
};

struct Header {
    id: u16,
    flags: Flags,
    qd_count: u16,
    an_count: u16,
    ns_count: u16,
    ar_count: u16,
}

struct Flags {
    query_response: QueryResponse,
    opcode: OpCode,
    authoritative_answer: bool,
    truncation: bool,
    recursion_denied: bool,
    recursion_avaliable: bool,
    // z - "Reserved for future use. Must be zero in all queries and responses" RFC1035
    response_code: ResponseCode,
}

#[derive(Debug, PartialEq)]
enum QueryResponse {
    Query,
    Response,
}

#[derive(Debug, PartialEq)]
enum OpCode {
    Query,
    InverseQuery,
    ServerStatusRequest,
    Reserved(u8),
}

#[derive(Debug, PartialEq)]
enum ResponseCode {
    NoError,
    FormatError,    // name server unable to interpret query
    ServerFailure,  // name server unable to process query due to internal error
    NameError,      // if from an authoritative name server, the queried name does not exist
    NotImplemented, // queried name server does not support request
    Refused,        // queried name server refuses request for policy reasons.
    Reserved(u8),
}

fn id(input: &[u8]) -> IResult<&[u8], u16> {
    be_u16(input)
}

fn qr(input: (&[u8], usize)) -> IResult<(&[u8], usize), QueryResponse> {
    bool(input).map(|(remaining, qr)| {
        (
            remaining,
            match qr {
                true => QueryResponse::Response,
                false => QueryResponse::Query,
            },
        )
    })
}

fn opcode(input: (&[u8], usize)) -> IResult<(&[u8], usize), OpCode> {
    take(4usize)(input).map(|(remaining, opcode)| {
        (
            remaining,
            match opcode {
                0 => OpCode::Query,
                1 => OpCode::InverseQuery,
                2 => OpCode::ServerStatusRequest,
                x if (3..16).contains(&x) => OpCode::Reserved(x),
                _ => unreachable!(),
            },
        )
    })
}

fn aa(input: (&[u8], usize)) -> IResult<(&[u8], usize), bool> {
    bool(input)
}

fn tc(input: (&[u8], usize)) -> IResult<(&[u8], usize), bool> {
    bool(input)
}

fn rd(input: (&[u8], usize)) -> IResult<(&[u8], usize), bool> {
    bool(input)
}

fn ra(input: (&[u8], usize)) -> IResult<(&[u8], usize), bool> {
    bool(input)
}

fn z(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    verify(take(3usize), |z: &u8| z == &0)(input)
}

fn rcode(input: (&[u8], usize)) -> IResult<(&[u8], usize), ResponseCode> {
    take(4usize)(input).map(|(remaining, rcode)| {
        (
            remaining,
            match rcode {
                0 => ResponseCode::NoError,
                1 => ResponseCode::FormatError,
                2 => ResponseCode::ServerFailure,
                3 => ResponseCode::NameError,
                4 => ResponseCode::NotImplemented,
                5 => ResponseCode::Refused,
                x if (6..16).contains(&x) => ResponseCode::Reserved(x),
                _ => unreachable!(),
            },
        )
    })
}

//fn flags(input: &[u8]) -> IResult<&[u8], (bool, u8, bool, bool, bool, bool, u8, u8)> {
//    pair(qr_opcode_aa_tc_rd, ra_z_rcode)(input)
//        .map(|(remaining, ((a, b, c, d, e), (f, g, h)))| (remaining, (a, b, c, d, e, f, g, h)))
//}

fn qd_count(input: &[u8]) -> IResult<&[u8], u16> {
    be_u16(input)
}

fn an_count(input: &[u8]) -> IResult<&[u8], u16> {
    be_u16(input)
}

fn ns_count(input: &[u8]) -> IResult<&[u8], u16> {
    be_u16(input)
}

fn ar_count(input: &[u8]) -> IResult<&[u8], u16> {
    be_u16(input)
}

#[cfg(test)]
mod tests {
    use proptest::{bits::u8::masked, prelude::*};

    proptest! {
        #[test]
        fn id(a in [0u8..255u8, 0u8..255u8]) {
            if let Ok((remaining, id)) = super::id(&a) {
                prop_assert_eq!(id, u16::from_be_bytes(a));
                prop_assert!(remaining.is_empty());
            }
        }
    }

    proptest! {
        #[test]
        fn qr(a in [arb_qr()]) {
            if let Ok(((remaining, ptr), qr)) = super::qr((&a, 0)) {
                prop_assert_eq!(qr, match a[0] != 0 {
                    true => super::QueryResponse::Response,
                    false => super::QueryResponse::Query,
        }) ;
                prop_assert_eq!(remaining, a);
                prop_assert_eq!(ptr, 1);
            }
        }
    }

    proptest! {
        #[test]
        fn opcode(a in [arb_opcode()]) {
            if let Ok(((remaining, ptr), opcode)) = super::opcode((&a, 1)) {
                prop_assert_eq!(opcode, match a[0] >> 3 {
                    0 => super::OpCode::Query,
                    1 => super::OpCode::InverseQuery,
                    2 => super::OpCode::ServerStatusRequest,
                    x if (3..16).contains(&x) => crate::header::OpCode::Reserved(x),
                    _ => unreachable!(),
        });
                prop_assert_eq!(remaining, a);
                prop_assert_eq!(ptr, 5);
            }
        }
    }

    proptest! {
        #[test]
        fn aa(a in [arb_aa()]) {
            if let Ok(((remaining, ptr), aa)) = super::aa((&a, 5)) {
                prop_assert_eq!(aa, a[0] != 0) ;
                prop_assert_eq!(remaining, a);
                prop_assert_eq!(ptr, 6);
            }
        }
    }

    proptest! {
        #[test]
        fn tc(a in [arb_tc()]) {
            if let Ok(((remaining, ptr), tc)) = super::tc((&a, 6)) {
                prop_assert_eq!(tc, a[0] != 0) ;
                prop_assert_eq!(remaining, a);
                prop_assert_eq!(ptr, 7);
            }
        }
    }

    proptest! {
        #[test]
        fn rd(a in [arb_rd()]) {
            if let Ok(((remaining, ptr), rd)) = super::rd((&a, 7)) {
                prop_assert_eq!(rd, a[0] != 0) ;
                prop_assert!(remaining.is_empty());
                prop_assert_eq!(ptr, 0);
            }
        }
    }

    proptest! {
        #[test]
        fn ra(a in [arb_ra()]) {
            if let Ok(((remaining, ptr), ra)) = super::ra((&a, 0)) {
                prop_assert_eq!(ra, a[0] != 0) ;
                prop_assert_eq!(remaining, a);
                prop_assert_eq!(ptr, 1);
            }
        }
    }

    proptest! {
        #[test]
        fn z(a in [arb_z()]) {
            if let Ok(((remaining, ptr), z)) = super::z((&a, 1)) {
                prop_assert_eq!(z, a[0] >> 4);
                prop_assert_eq!(remaining, a);
                prop_assert_eq!(ptr, 4);
            } else {
                prop_assert!((a[0] >> 4) > 0);
            }
        }
    }

    proptest! {
        #[test]
        fn rcode(a in [arb_rcode()]) {
            if let Ok(((remaining, ptr), rcode)) = super::rcode((&a, 4)) {
                prop_assert_eq!(rcode, match a[0] {
                0 => super::ResponseCode::NoError,
                1 => super::ResponseCode::FormatError,
                2 => super::ResponseCode::ServerFailure,
                3 => super::ResponseCode::NameError,
                4 => super::ResponseCode::NotImplemented,
                5 => super::ResponseCode::Refused,
                x if (6..16).contains(&x) => super::ResponseCode::Reserved(x),
                _ => unreachable!(),
        });
                prop_assert!(remaining.is_empty());
                prop_assert_eq!(ptr,0);
            }
        }
    }

    //    proptest! {
    //        #[test]
    //        fn flags_bytes(
    //            a in arb_qr(),
    //            b in arb_opcode(),
    //            c in arb_aa(),
    //            d in arb_tc(),
    //            e in arb_rd(),
    //            f in arb_ra(),
    //            g in arb_z(),
    //            h in arb_rcode())
    //        {
    //            let i = [a | b | c | d | e | f | g | h];
    //            if let Ok((remaining, (qr, opcode, aa, tc, rd, ra, z, rcode))) = super::flags(&i) {
    //                prop_assert_eq!(qr, a != 0);
    //                prop_assert_eq!(opcode, b >> 3);
    //                prop_assert_eq!(aa, c != 0) ;
    //                prop_assert_eq!(tc, d != 0) ;
    //                prop_assert_eq!(rd, e != 0) ;
    //                prop_assert!(remaining.is_empty());
    //                prop_assert_eq!(ra, f != 0);
    //                prop_assert_eq!(z, g >> 4);
    //                prop_assert_eq!(rcode, h) ;
    //                prop_assert!(remaining.is_empty());
    //            }
    //        }
    //    }

    prop_compose! {
        fn arb_qr()(qr in masked(0b1000_0000)) -> u8 {
            qr
        }
    }

    prop_compose! {
        fn arb_opcode()(opcode in masked(0b0111_1000)) -> u8 {
            opcode
        }
    }

    prop_compose! {
        fn arb_aa()(aa in masked(0b0000_0100)) -> u8 {
            aa
        }
    }

    prop_compose! {
        fn arb_tc()(tc in masked(0b0000_0010)) -> u8 {
            tc
        }
    }

    prop_compose! {
        fn arb_rd()(rd in masked(0b0000_0001)) -> u8 {
            rd
        }
    }

    prop_compose! {
        fn arb_ra()(ra in masked(0b1000_0000)) -> u8 {
            ra
        }
    }

    prop_compose! {
        fn arb_z()(z in masked(0b0111_0000)) -> u8 {
            z
        }
    }

    prop_compose! {
        fn arb_rcode()(rcode in masked(0b0000_1111)) -> u8 {
            rcode
        }
    }
}
