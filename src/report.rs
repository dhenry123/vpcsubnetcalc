use crate::{models::{Subnet}};

pub fn separator(){
    println!("---------------------------");
}

pub fn report_as_text(init_capacity: &str,network: String,subnets: Vec<Subnet>) {
    separator();
    println!("VPC network CIDR     : {}",network);
    println!("Subnet(s)            : {}",subnets.len(),);
    println!("Capacities:          : [{}]",init_capacity);
    separator();
    for subnet in subnets {
    println!("VPC subnet N°{}", subnet.idx + 1);
    println!("----------->");
    println!("Network              : {:?}", subnet.network);
    println!("Mask                 : {:?}", subnet.mask);
    println!("prefix               : {:?}", subnet.prefix);
    println!("Capacity             : {:?}", subnet.capacity);
    println!("CIDR                 : {}", subnet.cidr);
    println!("Broadcast            : {:?}", subnet.broadcast);
    println!("First usable address : {:?}", subnet.first_usable);
    println!("Last usable address  : {:?}", subnet.last_usable);
    println!("---------------------------");
    }
}

#[derive(Debug)]
struct CidrInfo {
    cidr: String,
    total_ips: u64,
    usable_ips: u64,
}

/**
 * Total IP Addresses: The total number of IP addresses in a CIDR block can be calculated as 2(32−prefix) for IPv4.
 * Usable IP Addresses: The number of usable IP addresses is typically 2(32−prefix)−2 (subtracting 2 for the network and broadcast addresses)
 * except for certain cases: For /31 and /32, the usable IPs are equal to the total IPs.
 */
pub fn print_network_capacity() {
    // Define the table header
    let header = ["CIDR", "# of IP Addresses", "# of Usable IP Addresses"];
    
    // Define the table data using computed values, iterating from 32 down to 0
    let data: Vec<CidrInfo> = (0..=32).rev().map(|prefix| {
        let total_ips = 1 << (32 - prefix); // 2^(32 - prefix)
        let usable_ips = match prefix {
            0 => total_ips - 2, // /0
            1 => total_ips - 2, // /1
            31 => total_ips,    // /31
            32 => total_ips,    // /32
            _ => total_ips - 2, // All other prefixes
        };
        CidrInfo {
            cidr: format!("/{}", prefix),
            total_ips,
            usable_ips,
        }
    }).collect();

    // Print the header
    let max_size = 52;
    println!("{}", "-".repeat(max_size));
    println!("{:<5} {:<20} {:<25}", header[0], header[1], header[2]);
    println!("{}", "-".repeat(max_size));

    // Print each row of data
    for row in data.iter() {
        println!("{:<5} {:<20} {:<25}", row.cidr, row.total_ips, row.usable_ips);
    }
    // Print footer
    println!("{}", "-".repeat(max_size));
}
