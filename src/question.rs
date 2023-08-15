#![allow(dead_code)]
//                                    1  1  1  1  1  1
//      0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |                                               |
//    /                     QNAME                     /
//    /                                               /
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |                     QTYPE                     |
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |                     QCLASS                    |
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

use nom::{
    combinator::{recognize, verify},
    multi::{fold_many0, length_data},
    number::complete::be_u8,
    sequence::pair,
    IResult,
};

use crate::resource_record::{RecordClass, RecordType};

struct Question {
    question_name: [u8; 255], // 255 octets is maximum,will usually be less
    question_type: u16,
    question_class: u16,
}

enum QuestionType {
    Record(RecordType),     // Values 1 - 16
    ZoneTransferRequest,    // (252) `AXFR`
    MailBoxRecordRequest,   // (253) `MAILB`
    MailAgentRecordRequest, // (254) `MAILA`
    All,                    // (255) `*` A request for all records
}

enum QuestionClass {
    Record(RecordClass),
    Any, // (255) `*` any class
}

fn question_name(input: &[u8]) -> IResult<&[u8], &[u8]> {
    recognize(pair(
        verify(
            fold_many0(
                length_data(verify(be_u8, |s: &u8| dbg!(s) != &0 && s < &64)),
                || 1,
                |acc, item: &[u8]| acc + 1 + item.len() as u8,
            ),
            |count: &u8| dbg!(count) <= &255,
        ),
        be_u8,
    ))(input)
}
