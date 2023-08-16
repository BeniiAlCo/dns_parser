mod authoritative_answer;
mod message_kind;
mod opcode;
mod recursion_query;
mod recursion_response;
mod response_code;
mod truncation;
mod z;

#[derive(Debug, PartialEq)]
struct Flags {
    //query_response: QueryResponse,
    //opcode: Opcode,
    //authoritative_answer: bool,
    //truncation: bool,
    //recursion_desired: bool,
    //recursion_avaliable: bool,
    // z: N/A - "Reserved for future use. Must be zero in all queries and responses" RFC1035
    //response_code: ResponseCode,
}
