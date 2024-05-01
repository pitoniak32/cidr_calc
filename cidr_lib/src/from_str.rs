use std::{net::Ipv4Addr, str::FromStr};

use regex::Regex;

use crate::{cidr_info::CidrInfo, error::Error};

impl FromStr for CidrInfo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ip, cidr) = parse_ip_and_cidr(s.to_string())?;
        CidrInfo::new(ip, cidr)
    }
}

fn parse_ip(os: [&str; 4]) -> Result<Ipv4Addr, Error> {
    Ok(Ipv4Addr::from_str(&format!(
        "{}.{}.{}.{}",
        os[0], os[1], os[2], os[3]
    ))?)
}

fn parse_cidr(input: &str) -> Result<u8, Error> {
    let n = input.parse::<u8>()?;
    if n > 32 {
        return Err(Error::CidrOutOfRange(n));
    }
    Ok(n)
}

pub fn parse_ip_and_cidr(input: String) -> Result<(Ipv4Addr, u8), Error> {
    let re = Regex::new(r"^(?<octet_1>(\d){1,3})(\.|\-)(?<octet_2>(\d){1,3})(\.|\-)(?<octet_3>(\d){1,3})(\.|\-)(?<octet_4>(\d){1,3})(\/|\-)(?<cidr>(\d){1,2})$").unwrap();

    let Some(parts) = re.captures(&input) else {
        return Err(Error::InvalidFormat(input.to_string()));
    };

    Ok((
        parse_ip([
            &parts["octet_1"],
            &parts["octet_2"],
            &parts["octet_3"],
            &parts["octet_4"],
        ])?,
        parse_cidr(&parts["cidr"])?,
    ))
}

#[cfg(test)]
mod test {
    use super::parse_ip_and_cidr;
    use crate::{
        error::Error,
        from_str::{parse_cidr, parse_ip},
    };
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::net::Ipv4Addr;

    #[rstest]
    #[case(["255", "255", "255", "255"], Ipv4Addr::new(255, 255, 255, 255))]
    #[case(["1","1","1","1"], Ipv4Addr::new(1, 1, 1, 1))]
    #[case(["0","0","0","0"], Ipv4Addr::new(0, 0, 0, 0))]
    fn test_parse_ip(#[case] input: [&str; 4], #[case] expected: Ipv4Addr) -> Result<(), Error> {
        assert_eq!(parse_ip(input)?, expected);
        Ok(())
    }

    #[rstest]
    #[case::too_big_ip(["256","256","256","256"])]
    #[case::too_small_ip(["-1","-1","-1","-1"])]
    #[should_panic]
    fn test_parse_ip_invalid(#[case] input: [&str; 4]) {
        parse_ip(input).unwrap();
    }

    #[rstest]
    #[case("32", 32)]
    #[case("16", 16)]
    #[case("0", 0)]
    fn test_parse_cidr(#[case] input: &str, #[case] expected: u8) {
        assert_eq!(parse_cidr(input).unwrap(), expected);
    }

    #[rstest]
    #[case::too_big_cidr("256")]
    #[case::too_big_cidr("33")]
    #[case::too_small_cidr("-1")]
    #[should_panic]
    fn test_parse_cidr_invalid(#[case] input: &str) {
        parse_cidr(input).unwrap();
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
        assert_eq!(parse_ip_and_cidr(input.to_string()).unwrap(), expected);
    }

    #[rstest]
    #[case::too_big_cidr("0.0.0.0/33")]
    #[case::too_small_cidr("0.0.0.0/-1")]
    #[case::too_big_ip("256.256.256.256/1")]
    #[case::too_small_ip("-1.-1.-1.-1/1")]
    #[case::multi_format("0-0-0-0/33")]
    #[should_panic]
    fn test_parse_ip_cidr_string_invalid(#[case] input: &str) {
        // Arrange / Act / Assert
        let _ = parse_ip_and_cidr(input.to_string()).unwrap();
    }
}
