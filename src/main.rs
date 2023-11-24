use anyhow::Result;
use std::{env, net::Ipv4Addr, process::exit, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct CidrInfo {
    pub ip: Ipv4Addr,
    pub cidr: u8,
    pub mask_subnet: Ipv4Addr,
    pub mask_wildcard: Ipv4Addr,
    pub addr_start: Ipv4Addr,
    pub addr_end: Ipv4Addr,
    pub addr_network: Ipv4Addr,
    pub addr_broadcast: Ipv4Addr,
    pub hosts: u64,
}

impl CidrInfo {
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

        let mask_subnet = get_subnet_mask(cidr);
        let mask_wildcard = get_wildcard_mask(mask_subnet); 
        let addr_network = get_network_addr(mask_subnet, ip);
        // TODO: account for cases where the broadcast and network are mixed with the start and end
        let addr_start = get_first_host_addr(addr_network);
        let addr_broadcast = get_broadcast_addr(mask_wildcard, ip);
        // TODO: account for cases where the broadcast and network are mixed with the start and end
        let addr_end = get_last_host_addr(addr_broadcast);

        CidrInfo {
            ip,
            cidr,
            mask_subnet,
            mask_wildcard,
            addr_start,
            addr_end,
            addr_network,
            addr_broadcast,
            hosts: 0,
        }
    }
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

fn get_first_host_addr(addr_network: Ipv4Addr) -> Ipv4Addr {
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

    Ipv4Addr::new(0,0,0,0)
    // Force all host bits to be 1
    // if sub_o == u8::MAX {
    //     addr_s
    // } else {
    //     let ones = wc_o.count_ones();
    //     let val: u16 = ((1u16 << ones) - 1) + addr_s as u16;
    //     if val <= u8::MAX as u16{
    //         return val as u8
    //     }
    //
    //     panic!("octet value is too large for a u8");
    // }
}

fn get_last_host_addr(addr_broadcast: Ipv4Addr) -> Ipv4Addr {
    let addr_broadcast_octets = addr_broadcast.octets();
    Ipv4Addr::new(
        addr_broadcast_octets[0],
        addr_broadcast_octets[1],
        addr_broadcast_octets[2],
        addr_broadcast_octets[3] - 1,
    )
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

    use crate::CidrInfo;

    #[test]
    fn basic_cidr_0() {
        // Arrange
        let expected_addr_info = CidrInfo {
            ip: Ipv4Addr::new(0, 0, 0, 0),
            cidr: 0,
            mask_subnet: Ipv4Addr::new(0, 0, 0, 0),
            mask_wildcard: Ipv4Addr::new(255, 255, 255, 255),
            addr_start: Ipv4Addr::new(0, 0, 0, 1),
            addr_end: Ipv4Addr::new(255, 255, 255, 254),
            addr_network: Ipv4Addr::new(0, 0, 0, 0),
            addr_broadcast: Ipv4Addr::new(255, 255, 255, 255),
            hosts: 4_294_967_296,
        };

        // Act
        let result_addr_info = CidrInfo::new("0.0.0.0/0");

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
            addr_start: Ipv4Addr::new(10, 8, 0, 1),
            addr_end: Ipv4Addr::new(10, 15, 255, 254),
            addr_network: Ipv4Addr::new(10, 8, 0, 0),
            addr_broadcast: Ipv4Addr::new(10, 15, 255, 255),
            hosts: 524_288,
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
            addr_start: Ipv4Addr::new(10, 0, 0, 1),
            addr_end: Ipv4Addr::new(10, 0, 0, 254),
            addr_network: Ipv4Addr::new(10, 0, 0, 0),
            addr_broadcast: Ipv4Addr::new(10, 0, 0, 255),
            hosts: 254,
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
            addr_start: Ipv4Addr::new(10, 0, 0, 0),
            addr_end: Ipv4Addr::new(10, 0, 0, 1),
            addr_network: Ipv4Addr::new(10, 0, 0, 0),
            addr_broadcast: Ipv4Addr::new(10, 0, 0, 1),
            hosts: 2,
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
