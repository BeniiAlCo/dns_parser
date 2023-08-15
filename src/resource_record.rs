#![allow(dead_code)]
//      0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |                                               |
//    /                                               /
//    /                      NAME                     /
//    |                                               |
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |                      TYPE                     |
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |                     CLASS                     |
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |                      TTL                      |
//    |                                               |
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |                   RDLENGTH                    |
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
//    /                     RDATA                     /
//    /                                               /
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

struct ResourceRecord {
    name: [u8; 255],
    record_type: RecordType,
    record_class: RecordClass,
    time_to_live: u32,
    record_data_length: u16,
    record_data: [u8; 255], // length is a placeholder!
}

pub(crate) enum RecordType {
    A,                // (1) `A` a host address
    NameServer,       // (2) `NS` an authoritative name server
    MailDestination,  // (3) `MD` a mail destination (Obsolete - use MX)
    MailForwarder,    // (4) `MF` a mail forwarder (Obsolete - use MX)
    CananicalName,    // (5) `CNAME` the canonical name for an alias
    StartOfAuthority, // (6) `SOA` marks the start of a zone of authority
    MailBox,          // (7) `MB` a mailbox domain name (EXPERIMENTAL)
    MailGroup,        // (8) `MG` a mail group member (EXPERIMENTAL)
    MailRename,       // (9) `MR` a mail rename domain name (EXPERIMENTAL)
    Null,             // (10) `NULL` a null RR (EXPERIMENTAL)
    WellKnownService, // (11) `WKS` a well known service description
    Pointer,          // (12) `PTR` a domain name pointer
    HostInfo,         // (13) `HINFO` host information
    MailInfo,         // (14) `MINFO` mailbox or mail list information
    MailExchange,     // (15) `MX` mail exchange
    Text,             // (16) `TXT` text strings
}

pub(crate) enum RecordClass {
    Internet, // (1) `IN` the Internet
    CSNet,    // (2) `CS` the CSET class
    Chaos,    // (3) `CH` the CHAOS class
    Hesiod,   // (4) `HS` Hesiod
}
