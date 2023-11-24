use anyhow::Result;
use helpers::usage;
use std::env;

use crate::cider_info::CidrInfo;

mod cider_info;
mod helpers;
mod ip_class;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
    }

    if let Some(ip_and_cidr) = args.get(1) {
        let cidr_info = CidrInfo::new(ip_and_cidr);
        println!("{cidr_info}");
    } else {
        usage();
    };

    Ok(())
}
