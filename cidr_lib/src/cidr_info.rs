use std::{fmt::Display, net::Ipv4Addr};

use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    helpers::{
        get_broadcast_addr, get_first_host_addr, get_host_values, get_last_host_addr,
        get_network_addr, get_subnet_mask, get_wildcard_mask,
    },
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CidrInfo {
    pub ip: Ipv4Addr,
    pub cidr: u8,
    pub subnet_mask: Ipv4Addr,
    pub wildcard_mask: Ipv4Addr,
    pub first_host_addr: Ipv4Addr,
    pub last_host_addr: Ipv4Addr,
    pub usable_hosts: u64,
    pub network_addr: Ipv4Addr,
    pub broadcast_addr: Ipv4Addr,
    pub total_hosts: u64,
}

impl Display for CidrInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Network Summary
ip...............: {ip}
cidr.............: {cidr}
subnet_mask......: {subnet_mask}
wildcard_mask....: {wildcard_mask}
first_host_addr..: {first_host_addr}
last_host_addr...: {last_host_addr} 
usable_hosts.....: {usable_hosts}
network_addr.....: {network_addr}
broadcast_addr...: {broadcast_addr}
total_hosts......: {total_hosts}",
            ip = self.ip,
            cidr = self.cidr,
            subnet_mask = self.subnet_mask,
            wildcard_mask = self.wildcard_mask,
            first_host_addr = self.first_host_addr,
            last_host_addr = self.last_host_addr,
            usable_hosts = self.usable_hosts,
            network_addr = self.network_addr,
            broadcast_addr = self.broadcast_addr,
            total_hosts = self.total_hosts,
        )
    }
}

impl CidrInfo {
    pub fn new(ip: Ipv4Addr, cidr: u8) -> Result<Self, Error> {
        let (hosts_total, hosts_usable) = get_host_values(cidr);

        let mask_subnet = get_subnet_mask(cidr);
        let mask_wildcard = get_wildcard_mask(mask_subnet);
        let addr_network = get_network_addr(mask_subnet, ip);
        let addr_host_first = get_first_host_addr(addr_network, hosts_usable);
        let addr_broadcast = get_broadcast_addr(mask_wildcard, ip);
        let addr_host_last = get_last_host_addr(addr_broadcast, hosts_usable);

        Ok(CidrInfo {
            ip,
            cidr,
            subnet_mask: mask_subnet,
            wildcard_mask: mask_wildcard,
            first_host_addr: addr_host_first,
            last_host_addr: addr_host_last,
            network_addr: addr_network,
            broadcast_addr: addr_broadcast,
            usable_hosts: hosts_usable,
            total_hosts: hosts_total,
        })
    }
}

#[cfg(test)]
mod test {
    use std::{net::Ipv4Addr, str::FromStr};

    use pretty_assertions::assert_eq;

    use crate::cidr_info::CidrInfo;

    #[test]
    fn basic_cidr_0() {
        // Arrange
        let expected_addr_info = CidrInfo {
            ip: Ipv4Addr::new(0, 0, 0, 0),
            cidr: 0,
            subnet_mask: Ipv4Addr::new(0, 0, 0, 0),
            wildcard_mask: Ipv4Addr::new(255, 255, 255, 255),
            first_host_addr: Ipv4Addr::new(0, 0, 0, 1),
            last_host_addr: Ipv4Addr::new(255, 255, 255, 254),
            usable_hosts: 4_294_967_294,
            network_addr: Ipv4Addr::new(0, 0, 0, 0),
            broadcast_addr: Ipv4Addr::new(255, 255, 255, 255),
            total_hosts: 4_294_967_296,
        };

        // Act
        let result_addr_info = CidrInfo::new(Ipv4Addr::from_str("0.0.0.0").unwrap(), 0).unwrap();

        // Assert
        assert_eq!(result_addr_info, expected_addr_info);
    }

    #[test]
    fn basic_cidr_1() {
        // Arrange
        let expected_addr_info = CidrInfo {
            ip: Ipv4Addr::new(0, 0, 0, 0),
            cidr: 1,
            subnet_mask: Ipv4Addr::new(128, 0, 0, 0),
            wildcard_mask: Ipv4Addr::new(127, 255, 255, 255),
            first_host_addr: Ipv4Addr::new(0, 0, 0, 1),
            last_host_addr: Ipv4Addr::new(127, 255, 255, 254),
            usable_hosts: 2_147_483_646,
            network_addr: Ipv4Addr::new(0, 0, 0, 0),
            broadcast_addr: Ipv4Addr::new(127, 255, 255, 255),
            total_hosts: 2_147_483_648,
        };

        // Act
        let result_addr_info = CidrInfo::new(Ipv4Addr::from_str("0.0.0.0").unwrap(), 1).unwrap();

        // Assert
        assert_eq!(result_addr_info, expected_addr_info);
    }

    #[test]
    fn basic_cidr_11() {
        // Arrange
        let expected_addr_info = CidrInfo {
            ip: Ipv4Addr::new(255, 255, 255, 253),
            cidr: 11,
            subnet_mask: Ipv4Addr::new(255, 224, 0, 0),
            wildcard_mask: Ipv4Addr::new(0, 31, 255, 255),
            first_host_addr: Ipv4Addr::new(255, 224, 0, 1),
            last_host_addr: Ipv4Addr::new(255, 255, 255, 254),
            usable_hosts: 2_097_150,
            network_addr: Ipv4Addr::new(255, 224, 0, 0),
            broadcast_addr: Ipv4Addr::new(255, 255, 255, 255),
            total_hosts: 2_097_152,
        };

        // Act
        let result_addr_info =
            CidrInfo::new(Ipv4Addr::from_str("255.255.255.253").unwrap(), 11).unwrap();

        // Assert
        assert_eq!(result_addr_info, expected_addr_info);
    }
    #[test]
    fn basic_cidr_13() {
        // Arrange
        let expected_addr_info = CidrInfo {
            ip: Ipv4Addr::new(10, 8, 17, 0),
            cidr: 13,
            subnet_mask: Ipv4Addr::new(255, 248, 0, 0),
            wildcard_mask: Ipv4Addr::new(0, 7, 255, 255),
            first_host_addr: Ipv4Addr::new(10, 8, 0, 1),
            last_host_addr: Ipv4Addr::new(10, 15, 255, 254),
            usable_hosts: 524_286,
            network_addr: Ipv4Addr::new(10, 8, 0, 0),
            broadcast_addr: Ipv4Addr::new(10, 15, 255, 255),
            total_hosts: 524_288,
        };

        // Act
        let result_addr_info = CidrInfo::new(Ipv4Addr::from_str("10.8.17.0").unwrap(), 13).unwrap();

        // Assert
        assert_eq!(result_addr_info, expected_addr_info);
    }

    #[test]
    fn basic_cidr_24() {
        // Arrange
        let expected_addr_info = CidrInfo {
            ip: Ipv4Addr::new(10, 0, 0, 1),
            cidr: 24,
            subnet_mask: Ipv4Addr::new(255, 255, 255, 0),
            wildcard_mask: Ipv4Addr::new(0, 0, 0, 255),
            first_host_addr: Ipv4Addr::new(10, 0, 0, 1),
            last_host_addr: Ipv4Addr::new(10, 0, 0, 254),
            network_addr: Ipv4Addr::new(10, 0, 0, 0),
            broadcast_addr: Ipv4Addr::new(10, 0, 0, 255),
            usable_hosts: 254,
            total_hosts: 256,
        };

        // Act
        let result_addr_info = CidrInfo::new(Ipv4Addr::from_str("10.0.0.1").unwrap(), 24).unwrap();

        // Assert
        assert_eq!(result_addr_info, expected_addr_info);
    }

    #[test]
    fn basic_cidr_31() {
        // Arrange
        let expected_addr_info = CidrInfo {
            ip: Ipv4Addr::new(10, 0, 0, 1),
            cidr: 31,
            subnet_mask: Ipv4Addr::new(255, 255, 255, 254),
            wildcard_mask: Ipv4Addr::new(0, 0, 0, 1),
            first_host_addr: Ipv4Addr::new(10, 0, 0, 0),
            last_host_addr: Ipv4Addr::new(10, 0, 0, 1),
            network_addr: Ipv4Addr::new(10, 0, 0, 0),
            broadcast_addr: Ipv4Addr::new(10, 0, 0, 1),
            usable_hosts: 0,
            total_hosts: 2,
        };

        // Act
        let result_addr_info = CidrInfo::new(Ipv4Addr::from_str("10.0.0.1").unwrap(), 31).unwrap();

        // Assert
        assert_eq!(result_addr_info, expected_addr_info);
    }

    // #[test]
    // #[should_panic]
    // fn too_large_octet_ipv4() {
    //     let addr_info = CidrInfo::new("256.0.0.0/32");
    //     assert_eq!(addr_info.ip, Ipv4Addr::new(0, 0, 0, 0));
    // }
    //
    // #[test]
    // #[should_panic]
    // fn too_small_octet_ipv4() {
    //     let addr_info = CidrInfo::new("-1.0.0.0/32");
    //     assert_eq!(addr_info.ip, Ipv4Addr::new(0, 0, 0, 0));
    // }
    //
    // #[test]
    // #[should_panic]
    // fn too_large_cider() {
    //     let _ = CidrInfo::new("0.0.0.0/33");
    // }
    //
    // #[test]
    // #[should_panic]
    // fn too_small_cider() {
    //     let _ = CidrInfo::new("0.0.0.0/-1");
    // }
}
