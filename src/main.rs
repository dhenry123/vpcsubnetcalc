mod subnet_compute;
mod models;
mod report;
mod constants;

use clap::{CommandFactory, Parser};
use constants::AWS_CONSTRAINT_MASK;
use models::Args;
use report::print_network_capacity;


use crate::models::VpcResult;
use crate::subnet_compute::{
    get_vpc_netmask, prepare_subnets_capacity,
};
use crate::report::report_as_text;
use std::process::exit;


fn main() {
    let args = Args::parse();

    // -d
    if args.display_capacity {
        print_network_capacity();
        return;
    }

    // -n -s
    if args.net.is_none() || args.subnets.is_none() {
        let mut command = Args::command();
        command.print_long_help().unwrap(); // Display the help message
        return;
    }

    let arg_subnets = &args.subnets.clone().unwrap();

    let subnets = prepare_subnets_capacity(arg_subnets);

    // Full capacity could not be < 12
    if subnets.clone().iter().sum::<i32>() < 12 {
        println!("{}",AWS_CONSTRAINT_MASK);
        exit(1)
    }
    // Each subnet capacity could not be < 12
    for item in subnets.clone() {
        if item < 12 { // min 10 + 2 unusable
            println!("{}",AWS_CONSTRAINT_MASK);
            exit(1)
        }
    }

    // VPC cidr
    let cidr_result = get_vpc_netmask(subnets.iter().sum());
    let cidr = match cidr_result {
        Ok(value) =>  value,
        Err(error) => {
            println!("Error: {}", error);
            exit(1);
        }
    };

    // Compute subnets from the main network
    let network = format!("{}/{}", args.net.unwrap(), cidr); // checked so unwrap safe 
    match crate::subnet_compute::split_network(&*network, subnets.clone()) {
        Ok(result) => {
            // Report
            if !args.json {
                // test
                report_as_text(&arg_subnets,network, result);// checked so unwrap safe 
            }else {
                // json
                let json_result = VpcResult{vpc:network,number_of_subnets:subnets.len(),capacities:arg_subnets.to_string(),subnets:result};
                let json_string = serde_json::to_string(&json_result).unwrap();
                println!("{}", json_string);
            }
        },
        Err(e) => {
            println!("{:?}",e);
            std::process::exit(1);
        }
    }
}
