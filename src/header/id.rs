use nom::{number::complete::be_u16, IResult};

struct Id(u16);

impl Id {
    fn parse(input: &[u8]) -> IResult<&[u8], u16> {
        be_u16(input)
    }
}

#[cfg(test)]
mod tests {
    use super::Id;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn all_u16s_are_valid_ids(input: u16) -> bool {
        let slice = &input.to_be_bytes();
        Id::parse(slice).is_ok()
    }
}
