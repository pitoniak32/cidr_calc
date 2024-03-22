use anyhow::{anyhow, Result};
use std::{net::Ipv4Addr, str::FromStr, u8};

use crate::USAGE_MSG;

pub fn parse_ip(input: &str) -> Result<Ipv4Addr> {
    Ok(Ipv4Addr::from_str(input)?)
}

pub fn parse_cidr(input: &str) -> Result<u8> {
    let msg = "CIDR must be in range 0-32 inclusive";
    match input.parse::<u8>() {
        Ok(n) => {
            if n <= 32 {
                Ok(n)
            } else {
                Err(anyhow!(msg))
            }
        }
        Err(_) => Err(anyhow!(msg)),
    }
}

fn split_dotted(input: &str) -> Result<(String, String)> {
    let parts: Vec<_> = input.split('/').collect();

    if parts.len() != 2 {
        return Err(anyhow!(USAGE_MSG));
    }

    Ok((
        parts.first().unwrap().to_string(),
        parts.get(1).unwrap().to_string(),
    ))
}

fn split_dashed(input: &str) -> Result<(String, String)> {
    let parts: Vec<_> = input.splitn(6, '-').collect();

    if parts.len() != 5 {
        return Err(anyhow!(USAGE_MSG));
    }

    let part1 = parts[..4].join(".");
    let part2 = parts[4].to_string();

    Ok((part1, part2))
}

pub fn parse_ip_and_cidr(input: String) -> Result<(Ipv4Addr, u8)> {
    let parts: (String, String) =
        if input.matches('/').count() == 1 && input.matches('.').count() == 3 {
            split_dotted(&input)?
        } else if input.matches('-').count() == 4 {
            split_dashed(&input)?
        } else {
            return Err(anyhow!(USAGE_MSG));
        };

    Ok((parse_ip(&parts.0)?, parse_cidr(&parts.1)?))
}

pub fn get_subnet_mask(cidr: u8) -> Ipv4Addr {
    let wildcard_bits = 32 - cidr;
    let mask_bits: String = format!(
        "{}{}",
        "1".repeat(cidr.into()),
        "0".repeat(wildcard_bits.into())
    );

    Ipv4Addr::new(
        u8::from_str_radix(&mask_bits[..8], 2).expect("bits should only contain 0 or 1."),
        u8::from_str_radix(&mask_bits[8..16], 2).expect("bits should only contain 0 or 1."),
        u8::from_str_radix(&mask_bits[16..24], 2).expect("bits should only contain 0 or 1."),
        u8::from_str_radix(&mask_bits[24..32], 2).expect("bits should only contain 0 or 1."),
    )
}

pub fn get_wildcard_mask(mask_subnet: Ipv4Addr) -> Ipv4Addr {
    let mask_subnet_octets = mask_subnet.octets();
    Ipv4Addr::new(
        u8::MAX - mask_subnet_octets[0],
        u8::MAX - mask_subnet_octets[1],
        u8::MAX - mask_subnet_octets[2],
        u8::MAX - mask_subnet_octets[3],
    )
}

pub fn get_network_addr(mask_subnet: Ipv4Addr, ip: Ipv4Addr) -> Ipv4Addr {
    let mask_subnet_octets = mask_subnet.octets();
    let ip_octets = ip.octets();
    Ipv4Addr::new(
        mask_subnet_octets[0] & ip_octets[0],
        mask_subnet_octets[1] & ip_octets[1],
        mask_subnet_octets[2] & ip_octets[2],
        mask_subnet_octets[3] & ip_octets[3],
    )
}

pub fn get_first_host_addr(addr_network: Ipv4Addr, hosts_usable: u64) -> Ipv4Addr {
    if hosts_usable == 0 {
        return addr_network;
    }

    let addr_network_octets = addr_network.octets();
    Ipv4Addr::new(
        addr_network_octets[0],
        addr_network_octets[1],
        addr_network_octets[2],
        addr_network_octets[3] + 1,
    )
}

pub fn get_broadcast_addr(mask_wildcard: Ipv4Addr, ip: Ipv4Addr) -> Ipv4Addr {
    let mask_wildcard_octets = mask_wildcard.octets();
    let ip_octets = ip.octets();

    Ipv4Addr::new(
        ip_octets[0] | mask_wildcard_octets[0],
        ip_octets[1] | mask_wildcard_octets[1],
        ip_octets[2] | mask_wildcard_octets[2],
        ip_octets[3] | mask_wildcard_octets[3],
    )
}

pub fn get_last_host_addr(addr_broadcast: Ipv4Addr, hosts_usable: u64) -> Ipv4Addr {
    if hosts_usable == 0 {
        return addr_broadcast;
    }
    let addr_broadcast_octets = addr_broadcast.octets();
    Ipv4Addr::new(
        addr_broadcast_octets[0],
        addr_broadcast_octets[1],
        addr_broadcast_octets[2],
        addr_broadcast_octets[3] - 1,
    )
}

pub fn get_host_values(cidr: u8) -> (u64, u64) {
    let total = 1 << (32 - cidr);
    if total >= 2 {
        return (total, total - 2);
    }
    (total, 0)
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::net::Ipv4Addr;

    use crate::helpers::{
        get_broadcast_addr, get_first_host_addr, get_host_values, get_last_host_addr,
        get_network_addr, get_subnet_mask, get_wildcard_mask, parse_cidr, parse_ip,
        parse_ip_and_cidr, split_dashed, split_dotted,
    };

    #[rstest]
    #[case("255.255.255.255/24", ("255.255.255.255", "24"))]
    #[case("1.1.1.1/0", ("1.1.1.1", "0"))]
    #[case("0.0.0.0/1", ("0.0.0.0", "1"))]
    fn test_split_dotted(#[case] input: &str, #[case] expected: (&str, &str)) -> Result<()> {
        let (ip, cidr) = expected;
        assert_eq!(split_dotted(input)?, (ip.to_string(), cidr.to_string()));
        Ok(())
    }

    #[rstest]
    #[case("255-255-255-255-24", ("255.255.255.255", "24"))]
    #[case("1-1-1-1-0", ("1.1.1.1", "0"))]
    #[case("0-0-0-0-1", ("0.0.0.0", "1"))]
    fn test_split_dashed(#[case] input: &str, #[case] expected: (&str, &str)) -> Result<()> {
        let (ip, cidr) = expected;
        assert_eq!(split_dashed(input)?, (ip.to_string(), cidr.to_string()));
        Ok(())
    }

    #[rstest]
    #[case("255.255.255.255", Ipv4Addr::new(255, 255, 255, 255))]
    #[case("1.1.1.1", Ipv4Addr::new(1, 1, 1, 1))]
    #[case("0.0.0.0", Ipv4Addr::new(0, 0, 0, 0))]
    fn test_parse_ip(#[case] input: &str, #[case] expected: Ipv4Addr) -> Result<()> {
        assert_eq!(parse_ip(input)?, expected);
        Ok(())
    }

    #[rstest]
    #[case::too_big_ip("256.256.256.256")]
    #[case::too_small_ip("-1.-1.-1.-1")]
    #[should_panic(expected = "invalid IPv4 address syntax")]
    fn test_parse_ip_invalid(#[case] input: &str) {
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
    #[should_panic(expected = "CIDR must be in range 0-32 inclusive")]
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
    #[should_panic(expected = "CIDR must be in range 0-32 inclusive")]
    #[case::too_big_cidr("0.0.0.0/33")]
    #[should_panic(expected = "CIDR must be in range 0-32 inclusive")]
    #[case::too_small_cidr("0.0.0.0/-1")]
    #[should_panic(expected = "invalid IPv4 address syntax")]
    #[case::too_big_ip("256.256.256.256/1")]
    #[should_panic(expected = "invalid IPv4 address syntax")]
    #[case::too_small_ip("-1.-1.-1.-1/1")]
    #[should_panic(
        expected = "format must be X.X.X.X/X, or X-X-X-X-X (ex: 10.0.0.1/24, or 10-0-0-1-24)"
    )]
    #[case::multi_format("0-0-0-0/33")]
    fn test_parse_ip_cidr_string_invalid(#[case] input: &str) {
        // Arrange / Act / Assert
        let _ = parse_ip_and_cidr(input.to_string()).unwrap();
    }

    #[rstest]
    #[case(0, Ipv4Addr::new(0, 0, 0, 0))]
    #[case(8, Ipv4Addr::new(255, 0, 0, 0))]
    #[case(16, Ipv4Addr::new(255, 255, 0, 0))]
    #[case(24, Ipv4Addr::new(255, 255, 255, 0))]
    #[case(25, Ipv4Addr::new(255, 255, 255, 128))]
    #[case(32, Ipv4Addr::new(255, 255, 255, 255))]
    fn test_get_subnet_mask(#[case] input: u8, #[case] expected: Ipv4Addr) {
        // Arrange / Act / Assert
        assert_eq!(get_subnet_mask(input), expected);
    }

    #[rstest]
    #[case(Ipv4Addr::new(0, 0, 0, 0), Ipv4Addr::new(255, 255, 255, 255))]
    #[case(Ipv4Addr::new(255, 0, 0, 0), Ipv4Addr::new(0, 255, 255, 255))]
    #[case(Ipv4Addr::new(255, 255, 0, 0), Ipv4Addr::new(0, 0, 255, 255))]
    #[case(Ipv4Addr::new(255, 255, 255, 0), Ipv4Addr::new(0, 0, 0, 255))]
    #[case(Ipv4Addr::new(255, 255, 255, 128), Ipv4Addr::new(0, 0, 0, 127))]
    #[case(Ipv4Addr::new(255, 255, 255, 255), Ipv4Addr::new(0, 0, 0, 0))]
    fn test_get_wildcard_mask(#[case] input: Ipv4Addr, #[case] expected: Ipv4Addr) {
        // Arrange / Act / Assert
        assert_eq!(get_wildcard_mask(input), expected);
    }

    #[rstest]
    #[case(
        Ipv4Addr::new(0, 0, 0, 0),
        Ipv4Addr::new(1, 2, 3, 4),
        Ipv4Addr::new(0, 0, 0, 0)
    )]
    #[case(
        Ipv4Addr::new(255, 255, 255, 0),
        Ipv4Addr::new(1, 2, 3, 4),
        Ipv4Addr::new(1, 2, 3, 0)
    )]
    #[case(
        Ipv4Addr::new(255, 255, 255, 0),
        Ipv4Addr::new(1, 2, 3, 4),
        Ipv4Addr::new(1, 2, 3, 0)
    )]
    #[case(
        Ipv4Addr::new(255, 255, 0, 0),
        Ipv4Addr::new(1, 2, 3, 4),
        Ipv4Addr::new(1, 2, 0, 0)
    )]
    fn test_get_network_addr(
        #[case] input1: Ipv4Addr,
        #[case] input2: Ipv4Addr,
        #[case] expected: Ipv4Addr,
    ) {
        // Arrange / Act / Assert
        assert_eq!(get_network_addr(input1, input2), expected);
    }

    #[rstest]
    #[case(Ipv4Addr::new(1, 2, 3, 4), 1, Ipv4Addr::new(1, 2, 3, 5))]
    #[case(Ipv4Addr::new(1, 2, 3, 0), 1, Ipv4Addr::new(1, 2, 3, 1))]
    #[case(Ipv4Addr::new(0, 0, 0, 0), 1, Ipv4Addr::new(0, 0, 0, 1))]
    #[case(Ipv4Addr::new(10, 0, 0, 1), 0, Ipv4Addr::new(10, 0, 0, 1))]
    fn test_get_first_host_addr(
        #[case] input1: Ipv4Addr,
        #[case] input2: u64,
        #[case] expected: Ipv4Addr,
    ) {
        // Arrange / Act / Assert
        assert_eq!(get_first_host_addr(input1, input2), expected,);
    }

    #[rstest]
    #[case(
        Ipv4Addr::new(0, 0, 0, 127),
        Ipv4Addr::new(1, 2, 3, 4),
        Ipv4Addr::new(1, 2, 3, 127)
    )]
    #[case(
        Ipv4Addr::new(0, 255, 255, 255),
        Ipv4Addr::new(1, 2, 3, 4),
        Ipv4Addr::new(1, 255, 255, 255)
    )]
    #[case(
        Ipv4Addr::new(0, 0, 0, 1),
        Ipv4Addr::new(1, 2, 3, 4),
        Ipv4Addr::new(1, 2, 3, 5)
    )]
    fn test_get_broadcast_addr(
        #[case] input1: Ipv4Addr,
        #[case] input2: Ipv4Addr,
        #[case] expected: Ipv4Addr,
    ) {
        // Arrange / Act / Assert
        assert_eq!(get_broadcast_addr(input1, input2), expected);
    }

    #[rstest]
    #[case(Ipv4Addr::new(1, 2, 3, 255), 1, Ipv4Addr::new(1, 2, 3, 254))]
    #[case(Ipv4Addr::new(1, 2, 3, 127), 1, Ipv4Addr::new(1, 2, 3, 126))]
    #[case(Ipv4Addr::new(1, 255, 255, 255), 1, Ipv4Addr::new(1, 255, 255, 254))]
    #[case(Ipv4Addr::new(10, 0, 0, 0), 0, Ipv4Addr::new(10, 0, 0, 0))]
    fn test_get_last_host_addr(
        #[case] input1: Ipv4Addr,
        #[case] input2: u64,
        #[case] expected: Ipv4Addr,
    ) {
        // Arrange / Act / Assert
        assert_eq!(get_last_host_addr(input1, input2), expected);
    }

    #[rstest]
    #[case(1, (2_147_483_648, 2_147_483_646))]
    #[case(24, (256, 254))]
    #[case(32, (1, 0))]
    fn test_get_host_values(#[case] input: u8, #[case] expected: (u64, u64)) {
        assert_eq!(get_host_values(input), expected);
    }
}
