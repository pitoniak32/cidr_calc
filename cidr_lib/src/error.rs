pub const PARSE_FMT_MSG: &str = "X(.,-)X(.,-)X(.,-)X(/,-)X (ex: 10.0.0.1/24, 10-0-0-1-24)";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("CIDR must be unsigned integer in range 0-32 inclusive")]
    InvalidCidr(#[from] std::num::ParseIntError),

    #[error("CIDR must be in range 0-32 inclusive, you provided = {0}")]
    CidrOutOfRange(u8),

    #[error("IP must be a vaild Ipv4 Address: {}", PARSE_FMT_MSG)]
    InvalidIp(#[from] std::net::AddrParseError),

    #[error("{0} is not valid! {PARSE_FMT_MSG}")]
    InvalidFormat(String),

    #[error("Make sure you are using a consistent pattern: {}", PARSE_FMT_MSG)]
    Format(String),
}
