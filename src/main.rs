use std::fmt::Display;

use anyhow::Result;
use clap::{Parser, ValueEnum};

use crate::cider_info::CidrInfo;

mod cider_info;
mod helpers;

#[derive(Parser)]
#[command(author, version, about)]
/// Manage your terminal environment.
struct Cli {
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
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cidr_info = CidrInfo::new(&cli.ip_cidr);

    match cli.output {
        Output::text => {
            println!("{cidr_info}");
        }
        Output::json => {
            println!("{}", serde_json::to_string_pretty::<CidrInfo>(&cidr_info).unwrap())
        }
        Output::yaml => {
            println!("{}", serde_yaml::to_string::<CidrInfo>(&cidr_info).unwrap())
        },
    }

    Ok(())
}
