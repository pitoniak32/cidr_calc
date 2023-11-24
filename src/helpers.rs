use std::{net::Ipv4Addr, process::exit, str::FromStr};

pub fn parse_ip_cidr_string(ip_and_cidr: &str) -> (Ipv4Addr, u8) {
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

    (ip, cidr)
}

pub fn usage() {
    eprintln!("usage: cidr <ipv4>/<cidr>");
    exit(1);
}

pub fn get_subnet_mask(cidr: u8) -> Ipv4Addr {
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
    use pretty_assertions::assert_eq;
    use std::net::Ipv4Addr;

    use crate::helpers::{
        get_broadcast_addr, get_first_host_addr, get_host_values, get_last_host_addr,
        get_network_addr, get_subnet_mask, get_wildcard_mask, parse_ip_cidr_string,
    };

    #[test]
    fn test_parse_ip_cidr_string() {
        // Arrange / Act / Assert
        assert_eq!(
            parse_ip_cidr_string("0.0.0.0/0"),
            (Ipv4Addr::new(0, 0, 0, 0), 0)
        );
        assert_eq!(
            parse_ip_cidr_string("0.0.0.1/1"),
            (Ipv4Addr::new(0, 0, 0, 1), 1)
        );
        assert_eq!(
            parse_ip_cidr_string("192.168.1.0/24"),
            (Ipv4Addr::new(192, 168, 1, 0), 24)
        );
        assert_eq!(
            parse_ip_cidr_string("255.255.255.255/32"),
            (Ipv4Addr::new(255, 255, 255, 255), 32)
        );
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
}
