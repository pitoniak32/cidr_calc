pub const USAGE_MSG: [&str; 4] = [
    "format must be ",
    DOTTED_USAGE_MSG,
    ", or ",
    DASHED_USAGE_MSG,
];
const DOTTED_USAGE_MSG: &str = "X.X.X.X/X (ex: 10.0.0.1/24)";
const DASHED_USAGE_MSG: &str = "X-X-X-X-X (ex. 10-0-0-1-24)";

#[derive(thiserror::Error, Debug)]
pub enum CalcError {
    #[error("CIDR must be unsigned integer in range 0-32 inclusive")]
    InvalidCidr(#[from] std::num::ParseIntError),

    #[error("CIDR must be in range 0-32 inclusive: provided = {0}")]
    CidrOutOfRange(u8),

    #[error("IP must be a vaild Ipv4 Address: {}", USAGE_MSG.concat())]
    InvalidIp(#[from] std::net::AddrParseError),

    #[error("Make sure you are using a consistent pattern: {DOTTED_USAGE_MSG}")]
    DottedSplit(String),

    #[error("Make sure you are using a consistent pattern: {DASHED_USAGE_MSG}")]
    DashedSplit(String),

    #[error("Make sure you are using a consistent pattern: {}", USAGE_MSG.concat())]
    Format(String),
}
