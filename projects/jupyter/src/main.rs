// Copyright 2020 The Evcxr Authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE
// or https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[macro_use]
extern crate json;

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;

mod connection;
mod control_file;
mod core;
mod install;
mod jupyter_message;

use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct JupyterApplication {
    /// Optional name to operate on
    name: Option<String>,
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    #[command(subcommand)]
    command: Option<JupyterCommands>,
}

#[derive(Subcommand)]
enum JupyterCommands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    Install(Box<InstallAction>),
    Uninstall(Box<UninstallAction>),
}

#[derive(Parser)]
pub struct InstallAction {
    /// Optional name to operate on
    name: Option<String>,
}

#[derive(Parser)]
pub struct UninstallAction {
    /// Optional name to operate on
    name: Option<String>,
}

fn run(control_file_name: &str) -> Result<()> {
    let config = control_file::Control::parse_file(control_file_name)?;
    core::Server::run(&config)
}





fn main() -> JupyterResult<()> {
    let cli = JupyterApplication::parse();
    // evcxr::runtime_hook();
    // let mut args = std::env::args();
    // let bin = args.next().unwrap();
    // if let Some(arg) = args.next() {
    //     match arg.as_str() {
    //         "--control_file" => {
    //             if let Err(error) = install::update_if_necessary() {
    //                 eprintln!("Warning: tried to update client, but failed: {}", error);
    //             }
    //             return run(&args.next().ok_or_else(|| anyhow!("Missing control file"))?);
    //         }
    //         "--install" => return install::install(),
    //         "--uninstall" => return install::uninstall(),
    //         "--open" => return install::open(),
    //
    //         "--help" => {}
    //         x => panic!("Unrecognised option {}", x),
    //     }
    // }
    // println!("To install, run:\n  {} --install", bin);
    // println!("To uninstall, run:\n  {} --uninstall", bin);
    Ok(())
}
