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
    IResult, Parser,
};

mod flags;
mod id;

#[derive(Debug, PartialEq)]
struct Header {
    id: u16,
    //flags: Flags,
    question_count: u16,
    answer_count: u16,
    nameserver_record_count: u16,
    additional_record_count: u16,
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
    Reserved(u8),
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

fn id(input: &[u8]) -> IResult<&[u8], u16> {
    be_u16(input)
}

fn query_response(input: (&[u8], usize)) -> IResult<(&[u8], usize), QueryResponse> {
    bool.map(|query_response| query_response.into())
        .parse(input)
}

fn opcode(input: (&[u8], usize)) -> IResult<(&[u8], usize), Opcode> {
    take::<_, u8, _, _>(4usize)
        .map(|op_code| op_code.into())
        .parse(input)
}

fn authoritative_answer(input: (&[u8], usize)) -> IResult<(&[u8], usize), bool> {
    bool(input)
}

fn truncation(input: (&[u8], usize)) -> IResult<(&[u8], usize), bool> {
    bool(input)
}

fn recursion_desired(input: (&[u8], usize)) -> IResult<(&[u8], usize), bool> {
    bool(input)
}

fn recursion_avaliable(input: (&[u8], usize)) -> IResult<(&[u8], usize), bool> {
    bool(input)
}

fn z(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    verify(take(3usize), |z: &u8| z == &0)(input)
}

fn response_code(input: (&[u8], usize)) -> IResult<(&[u8], usize), ResponseCode> {
    take::<_, u8, _, _>(4usize)
        .map(|response_code| response_code.into())
        .parse(input)
}

//fn//flags(input: &[u8]) -> IResult<&[u8], Flags> {
///   bits::<_, _, Error<(&[u8], usize)>, _, _>(tuple((
///       query_response,
///       opcode,
///       authoritative_answer,
///       truncation,
///       recursion_desired,
///       recursion_avaliable,
///       z,
///       response_code,
///   )))
///   .map(
///       |(
///           query_response,
///           opcode,
///           authoritative_answer,
///           truncation,
///           recursion_desired,
///           recursion_avaliable,
///           _z,
///           response_code,
///       )| Flags {
///           query_response,
///           opcode,
///           authoritative_answer,
///           truncation,
///           recursion_desired,
///           recursion_avaliable,
///           response_code,
///       },
///   )
///   .parse(input)
//}

fn question_count(input: &[u8]) -> IResult<&[u8], u16> {
    be_u16(input)
}

fn answer_count(input: &[u8]) -> IResult<&[u8], u16> {
    be_u16(input)
}

fn nameserver_record_count(input: &[u8]) -> IResult<&[u8], u16> {
    be_u16(input)
}

fn additional_record_count(input: &[u8]) -> IResult<&[u8], u16> {
    be_u16(input)
}

//fn header(input: &[u8]) -> IResult<&[u8], Header> {
//   tuple((
//       id,
//       flags,
//       question_count,
//       answer_count,
//       nameserver_record_count,
//       additional_record_count,
//   ))
//   .map(
//       |(
//           id,
//           flags,
//           question_count,
//           answer_count,
//           nameserver_record_count,
//           additional_record_count,
//       )| Header {
//           id,
//           flags,
//           question_count,
//           answer_count,
//           nameserver_record_count,
//           additional_record_count,
//       },
//   )
//   .parse(input)
//

//#[cfg(test)]
//od tests {
//   use proptest::{bits::u8::masked, prelude::*};

//   proptest! {
//       #[test]
//       fn id(input in [arb_byte(), arb_byte()]) {
//           let (remaining, id) = super::id(&input).unwrap();
//           let expected = u16::from_be_bytes(input);
//           prop_assert_eq!(id, expected);
//           prop_assert!(remaining.is_empty());
//       }
//   }

//   proptest! {
//       #[test]
//       fn query_response(query_response_bit in arb_query_response_bit()) {
//           let input = [query_response_bit];
//           let ((remaining, ptr), query_response) = super::query_response((&input, 0)).unwrap();
//           let expected = (query_response_bit != 0).into();
//           prop_assert_eq!(query_response, expected);
//           prop_assert_eq!(ptr, 1);
//           prop_assert_eq!(remaining, input);
//       }
//   }

//   proptest! {
//       #[test]
//       fn opcode(flags in [arb_opcode()]) {
//           let ((remaining, ptr), opcode) = super::opcode((&flags, 1)).unwrap();
//           let expected = (flags[0] >> 3).into();
//           prop_assert_eq!(opcode, expected);
//           prop_assert_eq!(ptr, 5);
//           prop_assert_eq!(remaining, flags);
//       }
//   }

//   proptest! {
//       #[test]
//       fn authoritative_answer(flags in [arb_authoritative_answer_bit()]) {
//           let ((remaining, ptr), authoritative_answer) = super::authoritative_answer((&flags, 5)).unwrap();
//           let expected = flags[0] != 0;
//           prop_assert_eq!(authoritative_answer, expected);
//           prop_assert_eq!(ptr, 6);
//           prop_assert_eq!(remaining, flags);
//       }
//   }

//   proptest! {
//       #[test]
//       fn tructation(flags in [arb_truncation_bit()]) {
//           let ((remaining, ptr), truncation) = super::truncation((&flags, 6)).unwrap();
//           let expected = flags[0] != 0;
//           prop_assert_eq!(truncation, expected);
//           prop_assert_eq!(ptr, 7);
//           prop_assert_eq!(remaining, flags);
//       }
//   }

//   proptest! {
//       #[test]
//       fn recursion_desired(flags in [arb_recursion_desired_bit()]) {
//           let ((remaining, ptr), recursion_desired) = super::recursion_desired((&flags, 7)).unwrap();
//           let expected = flags[0] != 0;
//           prop_assert_eq!(recursion_desired, expected);
//           prop_assert_eq!(ptr, 0);
//           prop_assert!(remaining.is_empty());
//       }
//   }

//   proptest! {
//       #[test]
//       fn recursion_avaliable(flags in [arb_recursion_avaliable_bit()]) {
//           let ((remaining, ptr), recursion_avaliable) = super::recursion_avaliable((&flags, 0)).unwrap();
//           let expected = flags[0] != 0;
//           prop_assert_eq!(recursion_avaliable, expected);
//           prop_assert_eq!(ptr, 1);
//           prop_assert_eq!(remaining, flags);
//       }
//   }

//   proptest! {
//       #[test]
//       fn z(flags in [arb_z()]) {
//           if flags[0] == 0 {
//               let ((remaining, ptr), z) = super::z((&flags, 1)).unwrap();
//               let expected = 0;
//               prop_assert_eq!(z, expected);
//               prop_assert_eq!(ptr, 4);
//               prop_assert_eq!(remaining, flags);
//           } else {
//               prop_assume!(super::z((&flags, 1)).is_err());
//           }
//       }
//   }

//   proptest! {
//       #[test]
//       fn response_code(flags in [arb_response_code()]) {
//           let ((remaining, ptr), response_code) = super::response_code((&flags, 4)).unwrap();
//           let expected = flags[0].into();
//           prop_assert_eq!(response_code, expected);
//           prop_assert!(remaining.is_empty());
//           prop_assert_eq!(ptr,0);
//       }
//   }

//   //   proptest! {
//   //       #[test]
//   //       fn flags(
//   //           query_response in arb_query_response_bit(),
//   //           opcode in arb_opcode(),
//   //           authoritative_answer in arb_authoritative_answer_bit(),
//   //           truncation in arb_truncation_bit(),
//   //           recursion_desired in arb_recursion_desired_bit(),
//   //           recursion_avaliable in arb_recursion_avaliable_bit(),
//   //           z in arb_z(),
//   //           response_code in arb_response_code(),
//   //       ) {
//   //           let first_flag_byte = query_response | opcode | authoritative_answer | truncation | recursion_desired;
//   //           let second_flag_byte = recursion_avaliable | z | response_code;
//   //           let input = [first_flag_byte, second_flag_byte];
//   //           if z == 0 {
//   //               let (remaining, flags) = super::flags(&input).unwrap();
//   //               let expected_query_response = (query_response != 0).into();
//   //               let expected_opcode = (opcode >> 3).into();
//   //               let expected_authoritative_answer = authoritative_answer != 0;
//   //               let expected_truncation = truncation != 0;
//   //               let expected_recursion_desired = recursion_desired != 0;
//   //               let expected_recursion_avaliable = recursion_avaliable != 0;
//   //               let expected_response_code = response_code.into();
//   //               let expected = super:: Flags {
//   //                   query_response: expected_query_response,
//   //                   opcode: expected_opcode,
//   //                   authoritative_answer: expected_authoritative_answer,
//   //                   truncation: expected_truncation,
//   //                   recursion_desired: expected_recursion_desired,
//   //                   recursion_avaliable: expected_recursion_avaliable,
//   //                   response_code: expected_response_code,
//   //               };
//   //               prop_assert_eq!(flags, expected);
//   //               prop_assert!(remaining.is_empty());
//   //           } else {
//   //               prop_assert!(super::flags(&input).is_err());
//   //           }
//   //       }
//   //   }

//   proptest! {
//       #[test]
//       fn question_count(
//           first_question_count_byte in arb_byte(),
//           second_question_count_byte in arb_byte()
//       ) {
//           let input = [first_question_count_byte, second_question_count_byte];
//           let (remaining, question_count) = super::question_count(&input).unwrap();
//           let expected = u16::from_be_bytes(input);
//           prop_assert_eq!(question_count, expected);
//           prop_assert!(remaining.is_empty());
//       }
//   }

//   proptest! {
//       #[test]
//       fn answer_count(
//           first_answer_count_byte in arb_byte(),
//           second_answer_count_byte in arb_byte()
//       ) {
//           let input = [first_answer_count_byte, second_answer_count_byte];
//           let (remaining, answer_count) = super::answer_count(&input).unwrap();
//           let expected = u16::from_be_bytes(input);
//           prop_assert_eq!(answer_count, expected);
//           prop_assert!(remaining.is_empty());
//       }
//   }

//   proptest! {
//       #[test]
//       fn nameserver_record_count(
//           first_nameserver_record_count_byte in arb_byte(),
//           second_nameserver_record_count_byte in arb_byte()
//       ) {
//           let input = [first_nameserver_record_count_byte, second_nameserver_record_count_byte];
//           let (remaining, nameserver_record_count) = super::nameserver_record_count(&input).unwrap();
//           let expected = u16::from_be_bytes(input);
//           prop_assert_eq!(nameserver_record_count, expected);
//           prop_assert!(remaining.is_empty());
//       }
//   }

//   proptest! {
//       #[test]
//       fn additional_record_count(
//           first_additional_record_count_byte in arb_byte(),
//           second_additional_record_count_byte in arb_byte()
//       ) {
//           let input = [first_additional_record_count_byte, second_additional_record_count_byte];
//           let (remaining, additional_record_count) = super::additional_record_count(&input).unwrap();
//           let expected = u16::from_be_bytes(input);
//           prop_assert_eq!(additional_record_count, expected);
//           prop_assert!(remaining.is_empty());
//       }
//   }

//   //   proptest! {
//   //       #[test]
//   //       fn header(
//   //           (first_id, second_id) in (arb_byte(), arb_byte()),
//   //           query_response in arb_query_response_bit(),
//   //           opcode in arb_opcode(),
//   //           authoritative_answer in arb_authoritative_answer_bit(),
//   //           truncation in arb_truncation_bit(),
//   //           recursion_desired in arb_recursion_desired_bit(),
//   //           recursion_avaliable in arb_recursion_avaliable_bit(),
//   //           z in arb_z(),
//   //           response_code in arb_response_code(),
//   //           (first_question_count, second_question_count) in (arb_byte(), arb_byte()),
//   //           (first_answer_count, second_answer_count) in (arb_byte(), arb_byte()),
//   //           (first_nameserver_record_count, second_nameserver_record_count) in (arb_byte(), arb_byte()),
//   //           (first_additional_record_count, second_additional_record_count) in (arb_byte(), arb_byte()),
//   //       ) {
//   //           let first_flag_byte = query_response | opcode | authoritative_answer | truncation | recursion_desired;
//   //           let second_flag_byte = recursion_avaliable | z | response_code;
//   //           let input = [
//   //               first_id,
//   //               second_id,
//   //               first_flag_byte,
//   //               second_flag_byte,
//   //               first_question_count,
//   //               second_question_count,
//   //               first_answer_count,
//   //               second_answer_count,
//   //               first_nameserver_record_count,
//   //               second_nameserver_record_count,
//   //               first_additional_record_count,
//   //               second_additional_record_count,
//   //           ];
//   //           if z == 0 {
//   //               let (remaining, header) = super::header(&input).unwrap();
//   //               let expected_id = u16::from_be_bytes([first_id, second_id]);
//   //               let expected_query_response = (query_response != 0).into();
//   //               let expected_opcode = (opcode >> 3).into();
//   //               let expected_authoritative_answer = authoritative_answer != 0;
//   //               let expected_truncation = truncation != 0;
//   //               let expected_recursion_desired = recursion_desired != 0;
//   //               let expected_recursion_avaliable = recursion_avaliable != 0;
//   //               let expected_response_code = response_code.into();
//   //               let expected_flags = super::Flags {
//   //                   query_response: expected_query_response,
//   //                   opcode: expected_opcode,
//   //                   authoritative_answer: expected_authoritative_answer,
//   //                   truncation: expected_truncation,
//   //                   recursion_desired: expected_recursion_desired,
//   //                   recursion_avaliable: expected_recursion_avaliable,
//   //                   response_code: expected_response_code,
//   //               };
//   //               let expected_question_count = u16::from_be_bytes([first_question_count, second_question_count]);
//   //               let expected_answer_count = u16::from_be_bytes([first_answer_count, second_answer_count]);
//   //               let expected_nameserver_record_count = u16::from_be_bytes([first_nameserver_record_count, second_nameserver_record_count]);
//   //               let expected_additional_record_count = u16::from_be_bytes([first_additional_record_count, second_additional_record_count]);
//   //               let expected = super::Header {
//   //                   id: expected_id,
//   //                   flags: expected_flags,
//   //                   question_count: expected_question_count,
//   //                   answer_count: expected_answer_count,
//   //                   nameserver_record_count: expected_nameserver_record_count,
//   //                   additional_record_count: expected_additional_record_count,
//   //               };
//   //               prop_assert_eq!(header, expected);
//   //               prop_assert!(remaining.is_empty());
//   //           } else {
//   //               prop_assert!(super::header(&input).is_err());
//   //           }
//   //       }
//   //   }

//   prop_compose! {
//       fn arb_byte()(id in 0u8..255u8) -> u8 {
//           id
//       }
//   }

//   prop_compose! {
//       fn arb_query_response_bit()(query_response in masked(0b1000_0000)) -> u8 {
//           query_response
//       }
//   }

//   prop_compose! {
//       fn arb_opcode()(opcode in masked(0b0111_1000)) -> u8 {
//           opcode
//       }
//   }

//   prop_compose! {
//       fn arb_authoritative_answer_bit()(authoritative_answer in masked(0b0000_0100)) -> u8 {
//           authoritative_answer
//       }
//   }

//   prop_compose! {
//       fn arb_truncation_bit()(truncation in masked(0b0000_0010)) -> u8 {
//           truncation
//       }
//   }

//   prop_compose! {
//       fn arb_recursion_desired_bit()(recursion_desired in masked(0b0000_0001)) -> u8 {
//           recursion_desired
//       }
//   }

//   prop_compose! {
//       fn arb_recursion_avaliable_bit()(recursion_avaliable in masked(0b1000_0000)) -> u8 {
//           recursion_avaliable
//       }
//   }

//   prop_compose! {
//       fn arb_z()(z in masked(0b0111_0000)) -> u8 {
//           z
//       }
//   }

//   prop_compose! {
//       fn arb_response_code()(response_code in masked(0b0000_1111)) -> u8 {
//           response_code
//       }
//   }
//
