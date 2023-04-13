//  -------------------------------------------------------------
//  Alkane
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
//  Project:        Nasqueron
//  License:        BSD-2-Clause
//  Description:    Manage nginx and php-fpm Alkane PaaS
//  -------------------------------------------------------------

#![feature(decl_macro)]

use std::process::exit;

use clap::Parser;

use crate::actions::*;
use crate::command::{AlkaneCommand, ToStatusCode};
use crate::config::AlkaneConfig;
use crate::deploy::DeployError;
use crate::runner::RecipeStatus;

//  -------------------------------------------------------------
//  Modules
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

mod actions;
mod command;
mod config;
mod db;
mod deploy;
mod runner;
mod server;
mod services;

//  -------------------------------------------------------------
//  Application entry point
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

fn main() {
    env_logger::init();

    let command = AlkaneCommand::parse(); //  Will exit if argument is missing or --help/--version provided.
    let config = match AlkaneConfig::load() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Can't load configuration: {:?}", error);
            exit(4);
        }
    };

    match command {
        AlkaneCommand::Server(args) => {
            serve(args, config);
        }

        AlkaneCommand::Update(args) => {
            let result = update(&args.site_name, None, &config);
            deploy_exit(result);
        }

        AlkaneCommand::Init(args) => {
            let result = initialize(&args.site_name, None, &config);
            deploy_exit(result);
        }

        AlkaneCommand::Deploy(args) => {
            let result = deploy(&args.site_name, None, &config);
            deploy_exit(result);
        }

        AlkaneCommand::IsPresent(args) => {
            let is_present = is_present(&args.site_name, &config);

            if !args.quiet {
                if is_present {
                    let path = config.get_site_path(&args.site_name).unwrap();
                    println!("{}", path);
                } else {
                    eprintln!("Site is absent.")
                }
            }

            exit(is_present.to_status_code());
        }
    }
}

fn deploy_exit(result: Result<RecipeStatus, DeployError>) {
    match result {
        Ok(status) => exit(status.to_status_code()),

        Err(error) => {
            eprintln!("{}", error);
            exit(16);
        }
    }
}
