pub const USAGE_MSG: &str =
    "format must be X.X.X.X/X (ex: 10.0.0.1/24), delimited by \".\", or \"-\"";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("CIDR must be unsigned integer in range 0-32 inclusive")]
    InvalidCidr(#[from] std::num::ParseIntError),

    #[error("CIDR must be in range 0-32 inclusive: provided = {0}")]
    CidrOutOfRange(u8),

    #[error("IP must be a vaild Ipv4 Address: {}", USAGE_MSG)]
    InvalidIp(#[from] std::net::AddrParseError),

    #[error("{0} is not valid! Make sure you are using a consistent pattern: {USAGE_MSG}")]
    InvalidFormat(String),

    #[error("Make sure you are using a consistent pattern: {}", USAGE_MSG)]
    Format(String),
}
