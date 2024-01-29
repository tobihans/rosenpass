use anyhow::bail;
use clap::{Parser, Subcommand};
use rosenpass::cli::Cli as RosenpassCli;
use std::{path::PathBuf, process::Command};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
#[command(propagate_version = true)]
struct Rp {
    // TODO: Use this option to configure the verbosity of the logger
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Genkey {
        private_keys_dir: PathBuf,
    },
    Pubkey {
        private_keys_dir: PathBuf,
        public_keys_dir: PathBuf,
    },
    // TODO: Add options and arguments for Exchange
    Exchange {},
}

impl Rp {
    fn run(self) -> anyhow::Result<()> {
        use Commands::*;

        match self.command {
            Genkey { private_keys_dir } => {
                let public_key = private_keys_dir.join("pqsk");
                let secret_key = private_keys_dir.join("pqpk");

                let cmd = RosenpassCli::GenKeys {
                    config_file: None,
                    public_key: Some(public_key),
                    secret_key: Some(secret_key),
                    force: false,
                };

                cmd.run()?;

                // TODO: Set file to 077
                // TODO: Use wireguard to gen key
                Ok(())
            }
            Pubkey {
                private_keys_dir,
                public_keys_dir,
            } => {
                println!(
                    "Generating public key in {:?} from private key in {:?}",
                    public_keys_dir, private_keys_dir
                );
                Ok(())
            }
            Exchange {} => {
                println!("Exchanging keys");
                Ok(())
            }
        }
    }
}

fn main() {
    // env_logger::init();
    let rp = Rp::parse().run();
    println!("{:?}", rp);
}
