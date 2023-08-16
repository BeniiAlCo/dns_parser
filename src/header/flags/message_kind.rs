use nom::{bits::complete::bool, IResult, Parser};

const INDEX: usize = 0;

#[derive(Debug, PartialEq)]
enum MessageKind {
    Query,    // (0) This message in a query
    Response, // (1) This message is a response
}

impl MessageKind {
    fn nom_parse(input: &[u8]) -> IResult<(&[u8], usize), MessageKind> {
        bool.map(|query_response: bool| query_response.into())
            .parse((input, INDEX))
    }

    fn parse(input: &[u8]) -> Result<(&[u8], MessageKind), String> {
        match Self::nom_parse(input) {
            Ok(((remaining, _index), qr)) => Ok((remaining, qr)),
            Err(_) => Err("Something went wrong :(".into()),
        }
    }
}

impl From<bool> for MessageKind {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Response,
            false => Self::Query,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MessageKind, INDEX};
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    #[derive(Debug, Clone)]
    struct MessageKindByte {
        value: u8,
        target: bool,
    }

    impl Arbitrary for MessageKindByte {
        fn arbitrary(g: &mut Gen) -> MessageKindByte {
            let input = u8::arbitrary(g);
            let target = bool::arbitrary(g);
            let mask = 0b0111_1111;
            let masked_input = input & mask;
            let bit_target = (target as u8) << 7;
            MessageKindByte {
                value: masked_input | bit_target,
                target,
            }
        }
    }

    #[quickcheck]
    fn all_u16s_are_valid_message_kinds(input: MessageKindByte) -> bool {
        let slice = &input.value.to_be_bytes();
        MessageKind::parse(slice).is_ok()
    }

    #[quickcheck]
    fn moves_index_forward_by_one(input: MessageKindByte) -> bool {
        let slice = &input.value.to_be_bytes();
        if let Ok(((_, new_index), _)) = MessageKind::nom_parse(slice) {
            new_index == INDEX + 1
        } else {
            false
        }
    }

    #[quickcheck]
    fn response_is_true_query_is_false(input: MessageKindByte) -> bool {
        let slice = &input.value.to_be_bytes();
        if let Ok((_, qr)) = MessageKind::parse(slice) {
            if input.target {
                qr == MessageKind::Response
            } else {
                qr == MessageKind::Query
            }
        } else {
            false
        }
    }

    #[quickcheck]
    fn does_not_consume_input(input: MessageKindByte) -> bool {
        let slice = &input.value.to_be_bytes();
        if let Ok((remaining, _)) = MessageKind::parse(slice) {
            remaining == slice
        } else {
            false
        }
    }
}
