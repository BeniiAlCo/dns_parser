use nom::{bits::complete::take, IResult, Parser};

const INDEX: usize = 1;

#[derive(Debug, PartialEq)]
enum Opcode {
    Query,               // (0) Standard: we have a name, we want an address
    InverseQuery,        // (1) Inverse: we have an address, we want a name
    ServerStatusRequest, // (2) Status: we want to know if the server is online
    Reserved(u8),        // (3-15) Reserved: reserved for future use
}

impl Opcode {
    fn nom_parse(input: &[u8]) -> IResult<(&[u8], usize), Opcode> {
        take::<_, u8, _, _>(4usize)
            .map(|op_code| op_code.into())
            .parse((input, INDEX))
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Query,
            1 => Self::InverseQuery,
            2 => Self::ServerStatusRequest,
            x if (3..=15).contains(&x) => Self::Reserved(x),
            _ => panic!("Opcode cannot exceed 15."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Opcode, INDEX};
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    #[derive(Debug, Clone)]
    struct OpcodeByte {
        value: [u8; 1],
        target: u8,
    }

    impl Arbitrary for OpcodeByte {
        fn arbitrary(g: &mut Gen) -> OpcodeByte {
            let input = u8::arbitrary(g);
            let target = u8::arbitrary(g);
            let input_mask = 0b1000_0111;
            let target_mask = 0b0111_1000;
            let masked_input = input & input_mask;
            let masked_target = target & target_mask;
            OpcodeByte {
                value: (masked_input | masked_target).to_be_bytes(),
                target: masked_target >> 3,
            }
        }
    }

    #[quickcheck]
    fn all_u8s_are_valid_opcodes(input: OpcodeByte) -> bool {
        Opcode::nom_parse(&input.value).is_ok()
    }

    #[quickcheck]
    fn moves_index_forward_by_four(input: OpcodeByte) -> bool {
        if let Ok(((_, new_index), _)) = Opcode::nom_parse(&input.value) {
            new_index == INDEX + 4
        } else {
            false
        }
    }

    #[quickcheck]
    fn opcode_value_matches_input_value(input: OpcodeByte) -> bool {
        if let Ok((_, opcode)) = Opcode::nom_parse(&input.value) {
            let expected = match input.target {
                0 => Opcode::Query,
                1 => Opcode::InverseQuery,
                2 => Opcode::ServerStatusRequest,
                x if (3..=15).contains(&x) => Opcode::Reserved(x),
                _ => return false,
            };
            opcode == expected
        } else {
            false
        }
    }

    #[quickcheck]
    fn does_not_consume_input(input: OpcodeByte) -> bool {
        if let Ok(((remaining, _), _)) = Opcode::nom_parse(&input.value) {
            remaining == input.value
        } else {
            false
        }
    }
}
