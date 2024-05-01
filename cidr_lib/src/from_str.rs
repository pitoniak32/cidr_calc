use std::{net::Ipv4Addr, str::FromStr};

use regex::Regex;

use crate::{cidr_info::CidrInfo, error::Error};

impl FromStr for CidrInfo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ip, cidr) = parse_ip_and_u8(s.to_string())?;
        CidrInfo::new(ip, cidr)
    }
}

fn parse_ip(ip: &str) -> Result<Ipv4Addr, Error> {
    Ok(Ipv4Addr::from_str(ip)?)
}

fn parse_ip_and_u8(input: String) -> Result<(Ipv4Addr, u8), Error> {
    let re = Regex::new(r"^(?<octet_1>(\d){1,3})(\.|\-)(?<octet_2>(\d){1,3})(\.|\-)(?<octet_3>(\d){1,3})(\.|\-)(?<octet_4>(\d){1,3})(\/|\-)(?<cidr>(-?\d){1,2})$").unwrap();

    let Some(parts) = re.captures(&input) else {
        return Err(Error::InvalidFormat(input.to_string()));
    };

    Ok((
        parse_ip(&format!(
            "{}.{}.{}.{}",
            &parts["octet_1"], &parts["octet_2"], &parts["octet_3"], &parts["octet_4"],
        ))?,
        parts["cidr"].parse::<u8>()?,
    ))
}

#[cfg(test)]
mod test {
    use super::parse_ip_and_u8;
    use crate::{error::Error, from_str::parse_ip};
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::net::Ipv4Addr;

    #[rstest]
    #[case("255.255.255.255", Ipv4Addr::new(255, 255, 255, 255))]
    #[case("1.1.1.1", Ipv4Addr::new(1, 1, 1, 1))]
    #[case("0.0.0.0", Ipv4Addr::new(0, 0, 0, 0))]
    fn test_parse_ip(#[case] input: &str, #[case] expected: Ipv4Addr) -> Result<(), Error> {
        assert_eq!(parse_ip(input)?, expected);
        Ok(())
    }

    #[rstest]
    #[case::too_big_ip("256.256.256.256")]
    #[case::too_small_ip("-1.-1.-1.-1")]
    #[should_panic]
    fn test_parse_ip_invalid(#[case] input: &str) {
        parse_ip(input).unwrap();
    }

    #[rstest]
    #[case("0.0.0.0/0", (Ipv4Addr::new(0, 0, 0, 0), 0))]
    #[case("0-0-0-0-0", (Ipv4Addr::new(0, 0, 0, 0), 0))]
    #[case("0.0.0.1/1", (Ipv4Addr::new(0, 0, 0, 1), 1))]
    #[case("0-0-0-1-1", (Ipv4Addr::new(0, 0, 0, 1), 1))]
    #[case("192.168.1.0/24", (Ipv4Addr::new(192, 168, 1, 0), 24))]
    #[case("192-168-1-0-24", (Ipv4Addr::new(192, 168, 1, 0), 24))]
    #[case("255.255.255.255/32", (Ipv4Addr::new(255, 255, 255, 255), 32))]
    #[case("255-255-255-255-32", (Ipv4Addr::new(255, 255, 255, 255), 32))]
    fn test_parse_ip_cidr_string(#[case] input: &str, #[case] expected: (Ipv4Addr, u8)) {
        // Arrange / Act / Assert
        assert_eq!(parse_ip_and_u8(input.to_string()).unwrap(), expected);
    }

    #[rstest]
    #[case::too_small_u8("0.0.0.0/-1")]
    #[case::too_big_ip("256.256.256.256/1")]
    #[case::too_small_ip("-1.-1.-1.-1/1")]
    #[should_panic]
    fn test_parse_ip_cidr_string_invalid(#[case] input: &str) {
        // Arrange / Act / Assert
        let _ = parse_ip_and_u8(input.to_string()).unwrap();
    }
}
