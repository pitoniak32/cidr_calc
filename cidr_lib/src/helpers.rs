use std::net::Ipv4Addr;

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
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::net::Ipv4Addr;

    use crate::helpers::{
        get_broadcast_addr, get_first_host_addr, get_host_values, get_last_host_addr,
        get_network_addr, get_subnet_mask, get_wildcard_mask,
    };

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
