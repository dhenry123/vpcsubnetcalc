use crate::models::Subnet;
use ipnetwork::Ipv4Network;
use std::{error::Error, net::Ipv4Addr};

pub fn get_vpc_netmask(hosts: i32) -> Result<u32, String> {
    let m = (hosts as f64).log(2.0).ceil() as u32;
    Ok(32 - m)
}

#[cfg(test)]
mod test_get_vpc_netmask {
    use super::*;

    #[test]
    fn test_get_vpc_netmask_for_250_hosts() {
        let result = get_vpc_netmask(250);
        assert_eq!(result, Ok(24));
    }

    #[test]
    fn test_get_vpc_netmask_for_5000_hosts() {
        let result = get_vpc_netmask(5000);
        assert_eq!(result, Ok(19));
    }

    #[test]
    fn test_get_vpc_netmask_for_10000_hosts() {
        let result = get_vpc_netmask(10000);
        assert_eq!(result, Ok(18));
    }
}

pub fn prepare_subnets_capacity(subnets: &str) -> Vec<i32> {
    let result: Vec<i32> = subnets
        // Replace commas with whitespace then split on whitespace
        .replace(',', " ") 
        .split_whitespace() 
        .filter_map(|s| {
            s.parse::<i32>()
                .map_err(|_| println!("Error: Only integer is supported"))
                .ok()
        }) // filtering out any invalid entries
        .map(|num| num + 2) // add unusable addresses: network & broadcast
        .collect();
    result
}

#[cfg(test)]
mod test_prepare_subnets_capacity {
    use super::*;

    #[test]
    fn prepare_subnets_capacity_3_10() {
        let result = prepare_subnets_capacity("10 10 10");
        // println!("size {:?}",result);
        assert_eq!(result, vec![12, 12, 12]);
    }

    #[test]
    fn prepare_subnets_capacity_4_10() {
        let result = prepare_subnets_capacity("10 10 10 10");
        // println!("size {:?}",result);
        assert_eq!(result, vec![12, 12, 12, 12]);
    }

    #[test]
    fn prepare_subnets_capacity_4_10_comma() {
        let result = prepare_subnets_capacity("10 10 10,10");
        // println!("size {:?}",result);
        assert_eq!(result, vec![12, 12, 12, 12]);
    }

    #[test]
    fn prepare_subnets_capacity_1_10() {
        let result = prepare_subnets_capacity("10");
        // println!("size {:?}",result);
        assert_eq!(result, vec![12]);
    }

    #[test]
    fn prepare_subnets_capacity_empty() {
        let result = prepare_subnets_capacity("");
        // println!("size {:?}",result);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn prepare_subnets_capacity_error_1() {
        let result = prepare_subnets_capacity("a");
        // println!("size {:?}",result);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn prepare_subnets_capacity_error_2() {
        let result = prepare_subnets_capacity("a b");
        // println!("size {:?}",result);
        assert_eq!(result, Vec::<i32>::new());
    }
}

pub(crate) fn split_network(network: &str, subnet_sizes: Vec<i32>) -> Result<Vec<Subnet>, Box<dyn Error>> {
    // Parse the network from CIDR notation
    let network: Ipv4Network = network.parse().map_err(|e| {
        format!("{} - Provided value: {}", e, network)
    })?;

    let mut subnets_vec = Vec::new();

    let mut count = 0;
    let mut current_address = network.ip();

    for size in subnet_sizes {
        // Calculate the required subnet mask for the given size
        let required_ips = size + 2; // Include network and broadcast addresses
        let subnet_mask = 32 - (required_ips as f32).log(2.0).ceil() as u8;
        let mut subnet: Option<Ipv4Network> = None;

        // Attempt to create a valid subnet
        while subnet.is_none() {
            // Create the subnet
            subnet = Ipv4Network::new(current_address, subnet_mask).ok();

            // If the subnet is invalid, increment the current address
            if let Some(ref valid_subnet) = subnet {
                if valid_subnet.network() < current_address {
                    // Increment current_address if the subnet's network address is less than current_address
                    current_address = Ipv4Addr::from(u32::from(current_address) + 1);
                    subnet = None; // Reset subnet to try again
                }
            }
        }

        // Fill the subnet object
        let valid_subnet = subnet.unwrap(); // Safe to unwrap since we have a valid subnet now
        subnets_vec.push(Subnet {
            idx: count,
            network: valid_subnet.network(),
            mask: valid_subnet.mask(),
            prefix: valid_subnet.prefix(),
            capacity: valid_subnet.iter().count() - 2, // Exclude network and broadcast
            cidr: format!("{:?}/{:?}", valid_subnet.network(), valid_subnet.prefix()),
            broadcast: valid_subnet.broadcast(),
            first_usable: Ipv4Addr::from(u32::from(valid_subnet.network()) + 1),
            last_usable: Ipv4Addr::from(u32::from(valid_subnet.broadcast()) - 1),
        });

        // Calculate the size of the current subnet
        let subnet_size = 1 << (32 - subnet_mask);

        // Move to the next available address
        current_address = Ipv4Addr::from(u32::from(current_address) + subnet_size);
        count += 1;
    }
    Ok(subnets_vec)
}

#[cfg(test)]
mod test_split_network {
    use super::*;

    #[test]
    fn split_network_3() {
        let result = split_network("192.168.1.0/26", vec![12, 12, 12]).unwrap();
        // println!("result {:?}", result);
        assert_eq!(result.len(), 3);
        assert_eq!(result.get(0).unwrap().cidr,"192.168.1.0/28");
        assert_eq!(result.get(1).unwrap().cidr,"192.168.1.16/28");
        assert_eq!(result.get(2).unwrap().cidr,"192.168.1.32/28");
    }

    #[test]
    fn split_network_4() {
        let result = split_network("172.28.1.0/26", vec![12, 12, 12, 12]).unwrap();
        // println!("result {:?}",result);
        assert_eq!(result.len(), 4);
        assert_eq!(result.get(0).unwrap().cidr,"172.28.1.0/28");
        assert_eq!(result.get(1).unwrap().cidr,"172.28.1.16/28");
        assert_eq!(result.get(2).unwrap().cidr,"172.28.1.32/28");
        assert_eq!(result.get(3).unwrap().cidr,"172.28.1.48/28");
    }

    #[test]
    fn split_network_5() {
        let result = split_network("172.28.1.0/26", vec![12, 12, 12, 12, 12]).unwrap();
        // println!("result {:?}",result);
        assert_eq!(result.len(), 5);
        assert_eq!(result.get(0).unwrap().cidr,"172.28.1.0/28");
        assert_eq!(result.get(1).unwrap().cidr,"172.28.1.16/28");
        assert_eq!(result.get(2).unwrap().cidr,"172.28.1.32/28");
        assert_eq!(result.get(3).unwrap().cidr,"172.28.1.48/28");
        assert_eq!(result.get(4).unwrap().cidr,"172.28.1.64/28");
    }
}
