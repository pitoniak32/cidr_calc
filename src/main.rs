use std::fmt::Display;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use helpers::parse_ip_and_cidr;

use crate::cidr_info::CidrInfo;

mod cidr_info;
mod helpers;

const USAGE_MSG: &str = "format must be X.X.X.X/X, or X-X-X-X-X (ex: 10.0.0.1/24, or 10-0-0-1-24)";

#[derive(Parser)]
#[command(author, version, about)]
/// Manage your terminal environment.
struct Cli {
    /// X.X.X.X/X, or X-X-X-X-X (ex: 10.0.0.1/24, or 10-0-0-1-24)
    ip_cidr: String,

    #[arg(short, long, default_value_t = Output::default())]
    output: Output,
}

#[derive(ValueEnum, Default, Clone, Debug)]
#[allow(non_camel_case_types)]
enum Output {
    #[default]
    text,
    json,
    yaml,
    yml,
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let (ip, cidr) = parse_ip_and_cidr(cli.ip_cidr)?;
    let cidr_info = CidrInfo::new(ip, cidr)?;

    match cli.output {
        Output::text => {
            println!("{cidr_info}");
        }
        Output::json => {
            println!(
                "{}",
                serde_json::to_string_pretty::<CidrInfo>(&cidr_info)
                    .expect("CidrInfo should be converted to valid json.")
            )
        }
        Output::yml | Output::yaml => {
            println!(
                "{}",
                serde_yaml::to_string::<CidrInfo>(&cidr_info)
                    .expect("CidrInfo should be converted to valid yaml.")
            )
        }
    }

    Ok(())
}
