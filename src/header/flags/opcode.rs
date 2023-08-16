use nom::{bits::complete::take, IResult, Parser};

#[derive(Debug, PartialEq)]
enum Opcode {
    Query,               // (0) Standard: we have a name, we want an address
    InverseQuery,        // (1) Inverse: we have an address, we want a name
    ServerStatusRequest, // (2) Status: we want to know if the server is online
    Reserved(u8),        // (3-15) Reserved: reserved for future use
}

impl Opcode {
    fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), Opcode> {
        take::<_, u8, _, _>(4usize)
            .map(|op_code| op_code.into())
            .parse(input)
    }
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
