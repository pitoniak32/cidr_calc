use std::{fmt::Display, str::FromStr};

use anyhow::Result;

use clap::{Parser, ValueEnum};

use cidr_lib::{cidr_info::CidrInfo, error::USAGE_MSG};

#[derive(Parser)]
#[command(author, version, about)]
/// Manage your terminal environment.
struct Cli {
    #[arg(help = USAGE_MSG)]
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
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cidr_info = CidrInfo::from_str(&cli.ip_cidr)?;

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
    }

    Ok(())
}
