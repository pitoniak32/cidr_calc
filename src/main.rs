use anyhow::Result;
use std::{env, net::Ipv4Addr, process::exit, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum IpClass {
    A,
    B,
    C,
    D,
    E,
}

#[derive(Debug, PartialEq)]
pub struct CidrInfo {
    pub ip: Ipv4Addr,
    pub cidr: u8,
    pub mask_subnet: Ipv4Addr,
    pub mask_wildcard: Ipv4Addr,
    pub addr_host_first: Ipv4Addr,
    pub addr_host_last: Ipv4Addr,
    pub hosts_usable: u64,
    pub addr_network: Ipv4Addr,
    pub addr_broadcast: Ipv4Addr,
    pub hosts_total: u64,
    pub ip_class: IpClass, 
}

impl CidrInfo {
    fn new(ip_and_cidr: &str) -> Self {
        let (ip, cidr) = parse_ip_cidr_string(ip_and_cidr);
        let (hosts_total, hosts_usable) = get_host_values(cidr);

        let mask_subnet = get_subnet_mask(cidr);
        let mask_wildcard = get_wildcard_mask(mask_subnet);
        let addr_network = get_network_addr(mask_subnet, ip);
        let addr_host_first = get_first_host_addr(addr_network, hosts_usable);
        let addr_broadcast = get_broadcast_addr(mask_wildcard, ip);
        let addr_host_last = get_last_host_addr(addr_broadcast, hosts_usable);
        let ip_class = get_ip_class(ip);

        CidrInfo {
            ip,
            cidr,
            mask_subnet,
            mask_wildcard,
            addr_host_first,
            addr_host_last,
            addr_network,
            addr_broadcast,
            hosts_usable,
            hosts_total,
            ip_class,
        }
    }
}

fn parse_ip_cidr_string(ip_and_cidr: &str) -> (Ipv4Addr, u8) {
    let parts: Vec<&str> = ip_and_cidr.split("/").collect();
    if parts.len() < 2 {
        usage();
    }
    let ip = Ipv4Addr::from_str(parts.get(0).expect("ip should not be none"))
        .expect("ip should be a vaild ipv4 address");
    let cidr = u8::from_str(parts.get(1).expect("cider mask should not be none"))
        .expect("CIDR should be in range 1-32 inclusive");
    if cidr < 1 || cidr > 32 {
        panic!("CIDR should be in range 1-32 inclusive");
    }

    (ip, cidr)
}

fn get_subnet_mask(cidr: u8) -> Ipv4Addr {
    let wildcard_bits = 32 - cidr;
    let mask_bits: String = format!(
        "{}{}",
        "1".repeat(cidr.into()).to_string(),
        "0".repeat(wildcard_bits.into())
    );

    Ipv4Addr::new(
        u8::from_str_radix(&mask_bits[..8], 2).unwrap(),
        u8::from_str_radix(&mask_bits[8..16], 2).unwrap(),
        u8::from_str_radix(&mask_bits[16..24], 2).unwrap(),
        u8::from_str_radix(&mask_bits[24..32], 2).unwrap(),
    )
}

fn get_wildcard_mask(mask_subnet: Ipv4Addr) -> Ipv4Addr {
    let mask_subnet_octets = mask_subnet.octets();
    Ipv4Addr::new(
        u8::MAX - mask_subnet_octets[0],
        u8::MAX - mask_subnet_octets[1],
        u8::MAX - mask_subnet_octets[2],
        u8::MAX - mask_subnet_octets[3],
    )
}

fn get_network_addr(mask_subnet: Ipv4Addr, ip: Ipv4Addr) -> Ipv4Addr {
    let mask_subnet_octets = mask_subnet.octets();
    let ip_octets = ip.octets();
    Ipv4Addr::new(
        mask_subnet_octets[0] & ip_octets[0],
        mask_subnet_octets[1] & ip_octets[1],
        mask_subnet_octets[2] & ip_octets[2],
        mask_subnet_octets[3] & ip_octets[3],
    )
}

fn get_first_host_addr(addr_network: Ipv4Addr, hosts_usable: u64) -> Ipv4Addr {
    if hosts_usable == 0 {
        return addr_network
    }

    let addr_network_octets = addr_network.octets();
    Ipv4Addr::new(
        addr_network_octets[0],
        addr_network_octets[1],
        addr_network_octets[2],
        addr_network_octets[3] + 1,
    )
}

fn get_broadcast_addr(mask_wildcard: Ipv4Addr, ip: Ipv4Addr) -> Ipv4Addr {
    let mask_wildcard_octets = mask_wildcard.octets();
    let ip_octets = ip.octets();

    Ipv4Addr::new(
        ip_octets[0] | mask_wildcard_octets[0],
        ip_octets[1] | mask_wildcard_octets[1],
        ip_octets[2] | mask_wildcard_octets[2],
        ip_octets[3] | mask_wildcard_octets[3],
    )
}

fn get_last_host_addr(addr_broadcast: Ipv4Addr, hosts_usable: u64) -> Ipv4Addr {
    if hosts_usable == 0 {
        return addr_broadcast
    }
    let addr_broadcast_octets = addr_broadcast.octets();
    Ipv4Addr::new(
        addr_broadcast_octets[0],
        addr_broadcast_octets[1],
        addr_broadcast_octets[2],
        addr_broadcast_octets[3] - 1,
    )
}

fn get_host_values(cidr: u8) -> (u64, u64) {
    let total = 1 << (32 - cidr);
    if total >= 2 {
        return (total, total - 2);
    }
    (total, 0)
}

fn get_ip_class(first_octet: Ipv4Addr) -> IpClass {
    match first_octet.octets()[0] {
        0..=127 => IpClass::A,
        128..=191 => IpClass::B,
        192..=223 => IpClass::C,
        224..=239 => IpClass::D,
        240..=255 => IpClass::E,
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
    }

    if let Some(ip_and_cidr) = args.get(1) {
        let addr_info = CidrInfo::new(ip_and_cidr);

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

    use crate::{
        get_broadcast_addr, get_first_host_addr, get_host_values, get_last_host_addr,
        get_network_addr, get_subnet_mask, get_wildcard_mask, CidrInfo, parse_ip_cidr_string, IpClass, get_ip_class,
    };

    #[test]
    fn test_parse_ip_cidr_string() {
        // Arrange / Act / Assert
        assert_eq!(parse_ip_cidr_string("0.0.0.1/1"), (Ipv4Addr::new(0, 0, 0, 1), 1));
        assert_eq!(parse_ip_cidr_string("192.168.1.0/24"), (Ipv4Addr::new(192, 168, 1, 0), 24));
        assert_eq!(parse_ip_cidr_string("255.255.255.255/32"), (Ipv4Addr::new(255, 255, 255, 255), 32));
    }

    #[test]
    #[should_panic]
    fn test_parse_ip_cidr_string_too_small_cidr() {
        // Arrange / Act / Assert
        parse_ip_cidr_string("0.0.0.0/0");
    }

    #[test]
    #[should_panic]
    fn test_parse_ip_cidr_string_too_big_cidr() {
        // Arrange / Act / Assert
        parse_ip_cidr_string("0.0.0.0/33");
    }
    
    #[test]
    #[should_panic]
    fn test_parse_ip_cidr_string_too_big_ip() {
        // Arrange / Act / Assert
        parse_ip_cidr_string("256.256.256.256/1");
    }

    #[test]
    #[should_panic]
    fn test_parse_ip_cidr_string_too_small_ip() {
        // Arrange / Act / Assert
        parse_ip_cidr_string("-1.-1.-1.-1/1");
    }

    #[test]
    fn test_get_subnet_mask() {
        // Arrange / Act / Assert
        assert_eq!(get_subnet_mask(0), Ipv4Addr::new(0, 0, 0, 0));
        assert_eq!(get_subnet_mask(8), Ipv4Addr::new(255, 0, 0, 0));
        assert_eq!(get_subnet_mask(16), Ipv4Addr::new(255, 255, 0, 0));
        assert_eq!(get_subnet_mask(24), Ipv4Addr::new(255, 255, 255, 0));
        assert_eq!(get_subnet_mask(25), Ipv4Addr::new(255, 255, 255, 128));
        assert_eq!(get_subnet_mask(32), Ipv4Addr::new(255, 255, 255, 255));
    }

    #[test]
    fn test_get_wildcard_mask() {
        // Arrange / Act / Assert
        assert_eq!(
            get_wildcard_mask(Ipv4Addr::new(0, 0, 0, 0)),
            Ipv4Addr::new(255, 255, 255, 255)
        );
        assert_eq!(
            get_wildcard_mask(Ipv4Addr::new(255, 0, 0, 0)),
            Ipv4Addr::new(0, 255, 255, 255)
        );
        assert_eq!(
            get_wildcard_mask(Ipv4Addr::new(255, 255, 0, 0)),
            Ipv4Addr::new(0, 0, 255, 255)
        );
        assert_eq!(
            get_wildcard_mask(Ipv4Addr::new(255, 255, 255, 0)),
            Ipv4Addr::new(0, 0, 0, 255)
        );
        assert_eq!(
            get_wildcard_mask(Ipv4Addr::new(255, 255, 255, 128)),
            Ipv4Addr::new(0, 0, 0, 127)
        );
        assert_eq!(
            get_wildcard_mask(Ipv4Addr::new(255, 255, 255, 255)),
            Ipv4Addr::new(0, 0, 0, 0)
        );
    }

    #[test]
    fn test_get_network_addr() {
        // Arrange / Act / Assert
        assert_eq!(
            get_network_addr(Ipv4Addr::new(0, 0, 0, 0), Ipv4Addr::new(1, 2, 3, 4)),
            Ipv4Addr::new(0, 0, 0, 0)
        );
        assert_eq!(
            get_network_addr(Ipv4Addr::new(255, 255, 255, 0), Ipv4Addr::new(1, 2, 3, 4)),
            Ipv4Addr::new(1, 2, 3, 0)
        );
        assert_eq!(
            get_network_addr(Ipv4Addr::new(255, 255, 0, 0), Ipv4Addr::new(1, 2, 3, 4)),
            Ipv4Addr::new(1, 2, 0, 0)
        );
    }

    #[test]
    fn test_get_first_host_addr() {
        // Arrange / Act / Assert
        assert_eq!(
            get_first_host_addr(Ipv4Addr::new(1, 2, 3, 4), 1),
            Ipv4Addr::new(1, 2, 3, 5)
        );
        assert_eq!(
            get_first_host_addr(Ipv4Addr::new(1, 2, 3, 0), 1),
            Ipv4Addr::new(1, 2, 3, 1)
        );
        assert_eq!(
            get_first_host_addr(Ipv4Addr::new(0, 0, 0, 0), 1),
            Ipv4Addr::new(0, 0, 0, 1)
        );
        assert_eq!(
            get_first_host_addr(Ipv4Addr::new(10, 0, 0, 1), 0),
            Ipv4Addr::new(10, 0, 0, 1)
        );
    }

    #[test]
    fn test_get_broadcast_addr() {
        // Arrange / Act / Assert
        assert_eq!(
            get_broadcast_addr(Ipv4Addr::new(0, 0, 0, 127), Ipv4Addr::new(1, 2, 3, 4)),
            Ipv4Addr::new(1, 2, 3, 127)
        );
        assert_eq!(
            get_broadcast_addr(Ipv4Addr::new(0, 255, 255, 255), Ipv4Addr::new(1, 2, 3, 4)),
            Ipv4Addr::new(1, 255, 255, 255)
        );
        assert_eq!(
            get_broadcast_addr(Ipv4Addr::new(0, 0, 0, 1), Ipv4Addr::new(1, 2, 3, 4)),
            Ipv4Addr::new(1, 2, 3, 5)
        );
    }

    #[test]
    fn test_get_last_host_addr() {
        // Arrange / Act / Assert
        assert_eq!(
            get_last_host_addr(Ipv4Addr::new(1, 2, 3, 255), 1),
            Ipv4Addr::new(1, 2, 3, 254)
        );
        assert_eq!(
            get_last_host_addr(Ipv4Addr::new(1, 2, 3, 127), 1),
            Ipv4Addr::new(1, 2, 3, 126)
        );
        assert_eq!(
            get_last_host_addr(Ipv4Addr::new(1, 255, 255, 255), 1),
            Ipv4Addr::new(1, 255, 255, 254)
        );
        assert_eq!(
            get_last_host_addr(Ipv4Addr::new(10, 0, 0, 0), 0),
            Ipv4Addr::new(10, 0, 0, 0)
        );
    }

    #[test]
    fn test_get_host_values() {
        assert_eq!(get_host_values(1), (2_147_483_648, 2_147_483_646));
        assert_eq!(get_host_values(24), (256, 254));
        assert_eq!(get_host_values(32), (1, 0));
    }

    #[test]
    fn test_get_ip_class() {
        assert_eq!(get_ip_class(Ipv4Addr::new(0,0,0,0)), IpClass::A);
        assert_eq!(get_ip_class(Ipv4Addr::new(127,0,0,0)), IpClass::A);
        assert_eq!(get_ip_class(Ipv4Addr::new(128,0,0,0)), IpClass::B);
        assert_eq!(get_ip_class(Ipv4Addr::new(191,0,0,0)), IpClass::B);
        assert_eq!(get_ip_class(Ipv4Addr::new(192,0,0,0)), IpClass::C);
        assert_eq!(get_ip_class(Ipv4Addr::new(223,0,0,0)), IpClass::C);
        assert_eq!(get_ip_class(Ipv4Addr::new(224,0,0,0)), IpClass::D);
        assert_eq!(get_ip_class(Ipv4Addr::new(239,0,0,0)), IpClass::D);
        assert_eq!(get_ip_class(Ipv4Addr::new(240,0,0,0)), IpClass::E);
        assert_eq!(get_ip_class(Ipv4Addr::new(255,0,0,0)), IpClass::E);
    }

    #[test]
    #[should_panic]
    fn basic_cidr_0() {
        // Arrange / Act / Assert
        CidrInfo::new("0.0.0.0/0");
    }

    #[test]
    fn basic_cidr_1() {
        // Arrange
        let expected_addr_info = CidrInfo {
            ip: Ipv4Addr::new(0, 0, 0, 0),
            cidr: 1,
            mask_subnet: Ipv4Addr::new(128, 0, 0, 0),
            mask_wildcard: Ipv4Addr::new(127, 255, 255, 255),
            addr_host_first: Ipv4Addr::new(0, 0, 0, 1),
            addr_host_last: Ipv4Addr::new(127, 255, 255, 254),
            hosts_usable: 2_147_483_646,
            addr_network: Ipv4Addr::new(0, 0, 0, 0),
            addr_broadcast: Ipv4Addr::new(127, 255, 255, 255),
            hosts_total: 2_147_483_648,
            ip_class: IpClass::A,
        };

        // Act
        let result_addr_info = CidrInfo::new("0.0.0.0/1");

        // Assert
        assert_eq!(result_addr_info, expected_addr_info);
    }

    #[test]
    fn basic_cidr_11() {
        // Arrange
        let expected_addr_info = CidrInfo {
            ip: Ipv4Addr::new(255, 255, 255, 253),
            cidr: 11,
            mask_subnet: Ipv4Addr::new(255, 224, 0, 0),
            mask_wildcard: Ipv4Addr::new(0, 31, 255, 255),
            addr_host_first: Ipv4Addr::new(255, 224, 0, 1),
            addr_host_last: Ipv4Addr::new(255, 255, 255, 254),
            hosts_usable: 2_097_150,
            addr_network: Ipv4Addr::new(255, 224, 0, 0),
            addr_broadcast: Ipv4Addr::new(255, 255, 255, 255),
            hosts_total: 2_097_152,
            ip_class: IpClass::E,
        };

        // Act
        let result_addr_info = CidrInfo::new("255.255.255.253/11");

        // Assert
        assert_eq!(result_addr_info, expected_addr_info);
    }
    #[test]
    fn basic_cidr_13() {
        // Arrange
        let expected_addr_info = CidrInfo {
            ip: Ipv4Addr::new(10, 8, 17, 0),
            cidr: 13,
            mask_subnet: Ipv4Addr::new(255, 248, 0, 0),
            mask_wildcard: Ipv4Addr::new(0, 7, 255, 255),
            addr_host_first: Ipv4Addr::new(10, 8, 0, 1),
            addr_host_last: Ipv4Addr::new(10, 15, 255, 254),
            hosts_usable: 524_286,
            addr_network: Ipv4Addr::new(10, 8, 0, 0),
            addr_broadcast: Ipv4Addr::new(10, 15, 255, 255),
            hosts_total: 524_288,
            ip_class: IpClass::A,
        };

        // Act
        let result_addr_info = CidrInfo::new("10.8.17.0/13");

        // Assert
        assert_eq!(result_addr_info, expected_addr_info);
    }

    #[test]
    fn basic_cidr_24() {
        // Arrange
        let expected_addr_info = CidrInfo {
            ip: Ipv4Addr::new(10, 0, 0, 1),
            cidr: 24,
            mask_subnet: Ipv4Addr::new(255, 255, 255, 0),
            mask_wildcard: Ipv4Addr::new(0, 0, 0, 255),
            addr_host_first: Ipv4Addr::new(10, 0, 0, 1),
            addr_host_last: Ipv4Addr::new(10, 0, 0, 254),
            addr_network: Ipv4Addr::new(10, 0, 0, 0),
            addr_broadcast: Ipv4Addr::new(10, 0, 0, 255),
            hosts_usable: 254,
            hosts_total: 256,
            ip_class: IpClass::A,
        };

        // Act
        let result_addr_info = CidrInfo::new("10.0.0.1/24");

        // Assert
        assert_eq!(result_addr_info, expected_addr_info);
    }

    #[test]
    fn basic_cidr_31() {
        // Arrange
        let expected_addr_info = CidrInfo {
            ip: Ipv4Addr::new(10, 0, 0, 1),
            cidr: 31,
            mask_subnet: Ipv4Addr::new(255, 255, 255, 254),
            mask_wildcard: Ipv4Addr::new(0, 0, 0, 1),
            addr_host_first: Ipv4Addr::new(10, 0, 0, 0),
            addr_host_last: Ipv4Addr::new(10, 0, 0, 1),
            addr_network: Ipv4Addr::new(10, 0, 0, 0),
            addr_broadcast: Ipv4Addr::new(10, 0, 0, 1),
            hosts_usable: 0,
            hosts_total: 2,
            ip_class: IpClass::A,
        };

        // Act
        let result_addr_info = CidrInfo::new("10.0.0.1/31");

        // Assert
        assert_eq!(result_addr_info, expected_addr_info);
    }

    #[test]
    #[should_panic]
    fn too_large_octet_ipv4() {
        let addr_info = CidrInfo::new("256.0.0.0/32");
        assert_eq!(addr_info.ip, Ipv4Addr::new(0, 0, 0, 0));
    }

    #[test]
    #[should_panic]
    fn too_small_octet_ipv4() {
        let addr_info = CidrInfo::new("-1.0.0.0/32");
        assert_eq!(addr_info.ip, Ipv4Addr::new(0, 0, 0, 0));
    }

    #[test]
    #[should_panic]
    fn too_large_cider() {
        let _ = CidrInfo::new("0.0.0.0/33");
    }

    #[test]
    #[should_panic]
    fn too_small_cider() {
        let _ = CidrInfo::new("0.0.0.0/-1");
    }
}
