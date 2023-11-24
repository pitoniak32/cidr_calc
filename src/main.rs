use anyhow::Result;
use std::{env, net::Ipv4Addr, process::exit, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct AddressInfo {
    pub ip: Ipv4Addr,
    pub cidr: u8,
    pub mask_subnet: Ipv4Addr,
    pub mask_wildcard: Ipv4Addr,
    pub addr_start: Ipv4Addr,
    pub addr_end: Ipv4Addr,
    pub addr_broadcast: Ipv4Addr,
    pub addr_network: Ipv4Addr,
    pub hosts: u64,
}

impl AddressInfo {
    fn new(ip_and_cidr: &str) -> Self {
        let parts: Vec<&str> = ip_and_cidr.split("/").collect();
        if parts.len() < 2 {
            usage();
        }

        let ip = Ipv4Addr::from_str(parts.get(0).expect("ip should not be none"))
            .expect("ip should be a vaild ipv4 address");
        let cidr = u8::from_str(parts.get(1).expect("cider mask should not be none"))
            .expect("CIDR should be in range 0-32 inclusive");
        if cidr > 32 {
            panic!("CIDR should be in range 0-32 inclusive");
        }

        // 32 bits
        // 11111111.11111111.11111111.11111111
        // 255.255.255.255
        //
        let wildcard_bits = 32 - cidr;
        let mask_bits: String = format!(
            "{}{}",
            "1".repeat(cidr.into()).to_string(),
            "0".repeat(wildcard_bits.into())
        );

        let mask_subnet = Ipv4Addr::new(
            u8::from_str_radix(&mask_bits[..8], 2).unwrap(),
            u8::from_str_radix(&mask_bits[8..16], 2).unwrap(),
            u8::from_str_radix(&mask_bits[16..24], 2).unwrap(),
            u8::from_str_radix(&mask_bits[24..32], 2).unwrap(),
        );
        let mask_subnet_octets = mask_subnet.octets();
        let mask_wildcard = Ipv4Addr::new( 
            u8::MAX - mask_subnet_octets[0],
            u8::MAX - mask_subnet_octets[1],
            u8::MAX - mask_subnet_octets[2],
            u8::MAX - mask_subnet_octets[3],
        );
        let ip_octets = ip.octets();

        let addr_start = Ipv4Addr::new(
            mask_subnet_octets[0] & ip_octets[0],
            mask_subnet_octets[1] & ip_octets[1],
            mask_subnet_octets[2] & ip_octets[2],
            mask_subnet_octets[3] & ip_octets[3],
        );

        let addr_end = Ipv4Addr::new(
            if mask_subnet_octets[0] == u8::MAX {
                ip_octets[0]
            } else {
                (1 << u8::MAX - mask_subnet_octets[0]) - 1 + ip_octets[0]
            },
            if mask_subnet_octets[1] == u8::MAX {
                ip_octets[1]
            } else {
                (1 << u8::MAX - mask_subnet_octets[1]) - 1 + ip_octets[1]
            },
            if mask_subnet_octets[2] == u8::MAX {
                ip_octets[2]
            } else {
                (1 << u8::MAX - mask_subnet_octets[2]) - 1 + ip_octets[2]
            },
            if mask_subnet_octets[3] == u8::MAX {
                ip_octets[3]
            } else {
                (1 << u8::MAX - mask_subnet_octets[3]) - 1 + ip_octets[3]
            },
        );

        AddressInfo {
            ip,
            cidr,
            mask_subnet,
            mask_wildcard,
            addr_start,
            addr_end,
            addr_broadcast: Ipv4Addr::new(0, 0, 0, 0),
            addr_network: Ipv4Addr::new(0, 0, 0, 0),
            hosts: 0,
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
    }

    if let Some(ip_and_cidr) = args.get(1) {
        let addr_info = AddressInfo::new(ip_and_cidr);

        println!("{:#?}", addr_info);
    };

    Ok(())
}

fn usage() {
    eprintln!("usage: cidr <ipv4>/<cidr>");
    exit(1);
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use std::net::Ipv4Addr;

    use crate::AddressInfo;

    #[test]
    fn basic_cidr_0() {
        // Arrange
        let expected_addr_info = AddressInfo {
            ip: Ipv4Addr::new(0, 0, 0, 0),
            cidr: 0,
            mask_subnet: Ipv4Addr::new(0, 0, 0, 0),
            mask_wildcard: Ipv4Addr::new(0, 0, 0, 0),
            addr_start: Ipv4Addr::new(0, 0, 0, 0),
            addr_end: Ipv4Addr::new(255, 255, 255, 255),
            addr_broadcast: Ipv4Addr::new(0, 0, 0, 0),
            addr_network: Ipv4Addr::new(0, 0, 0, 0),
            hosts: 4_294_967_296,
        };

        // Act
        let result_addr_info = AddressInfo::new("0.0.0.0/0");

        // Assert
        assert_eq!(result_addr_info, expected_addr_info);
    }

    #[test]
    fn basic_cidr_31() {
        // Arrange
        let expected_addr_info = AddressInfo {
            ip: Ipv4Addr::new(10, 0, 0, 1),
            cidr: 31,
            mask_subnet: Ipv4Addr::new(255, 255, 255, 254),
            mask_wildcard: Ipv4Addr::new(0, 0, 0, 1),
            addr_start: Ipv4Addr::new(10, 0, 0, 0),
            addr_end: Ipv4Addr::new(10, 0, 0, 1),
            addr_broadcast: Ipv4Addr::new(10, 0, 0, 1),
            addr_network: Ipv4Addr::new(10, 0, 0, 0),
            hosts: 2,
        };

        // Act
        let result_addr_info = AddressInfo::new("0.0.0.0/31");

        // Assert
        assert_eq!(result_addr_info, expected_addr_info);
    }

    #[test]
    fn basic_cidr_24() {
        // Arrange
        let expected_addr_info = AddressInfo {
            ip: Ipv4Addr::new(10, 0, 0, 1),
            cidr: 24,
            mask_subnet: Ipv4Addr::new(255, 255, 255, 0),
            mask_wildcard: Ipv4Addr::new(0, 0, 0, 255),
            addr_start: Ipv4Addr::new(10, 0, 0, 1),
            addr_end: Ipv4Addr::new(10, 0, 0, 254),
            addr_broadcast: Ipv4Addr::new(10, 0, 0, 255),
            addr_network: Ipv4Addr::new(10, 0, 0, 0),
            hosts: 254,
        };

        // Act
        let result_addr_info = AddressInfo::new("0.0.0.0/31");

        // Assert
        assert_eq!(result_addr_info, expected_addr_info);
    }

    #[test]
    #[should_panic]
    fn too_large_octet_ipv4() {
        let addr_info = AddressInfo::new("256.0.0.0/32");
        assert_eq!(addr_info.ip, Ipv4Addr::new(0, 0, 0, 0));
    }

    #[test]
    #[should_panic]
    fn too_small_octet_ipv4() {
        let addr_info = AddressInfo::new("-1.0.0.0/32");
        assert_eq!(addr_info.ip, Ipv4Addr::new(0, 0, 0, 0));
    }

    #[test]
    #[should_panic]
    fn too_large_cider() {
        let _ = AddressInfo::new("0.0.0.0/33");
    }

    #[test]
    #[should_panic]
    fn too_small_cider() {
        let _ = AddressInfo::new("0.0.0.0/-1");
    }
}
