use std::net::Ipv4Addr;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Subnet {
    pub idx: i32,
    pub network: Ipv4Addr,
    pub mask: Ipv4Addr,
    pub prefix: u8,
    pub capacity: usize,
    pub cidr: String,
    pub broadcast: Ipv4Addr,
    pub first_usable: Ipv4Addr,
    pub last_usable: Ipv4Addr,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VpcResult {
    pub vpc: String,
    pub number_of_subnets: usize,
    pub capacities: String,
    pub subnets: Vec<Subnet>
}

// Clap
#[derive(Parser, Debug)]
#[command(version, about, long_about = "Generates subnets from a given network address and specified capacities.\n\n(C)2025-Damien HENRY-<https://www.mytinydc.com>")]
pub struct Args {
    /// Base of the VPC network eg: 172.28.0.0 - cidr will be compute from subnet capacities
    #[arg(short, long)]
    pub net: Option<String>,

    /// Subnet capacities, separated by a space or comma, e.g. “10 20 20”.
    #[arg(short, long)]
    pub subnets: Option<String>,

    /// Output as json, default is text
    #[arg(long)]
    pub json: bool,

    /// Help about networks capacity
    #[arg(short = 'd', long)]
    pub display_capacity: bool,
}
