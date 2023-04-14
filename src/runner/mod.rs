//  -------------------------------------------------------------
//  Alkane :: Runner
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
//  Project:        Nasqueron
//  License:        BSD-2-Clause
//  Description:    Run a recipe to initialize or update a site
//  -------------------------------------------------------------

use std::ffi::OsStr;
use std::fmt::{Debug, Display};
use std::process::Command;

use log::{error, info, warn};
use serde::Serialize;

//  -------------------------------------------------------------
//  Modules
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub mod site;
pub mod store;

//  -------------------------------------------------------------
//  Exit status of a recipe.
//
//  The executable called to build the site should use
//  those exit code inspired by the Nagios one.
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Serialize)]
pub enum RecipeStatus {
    Success,
    Warning,
    Error,
    Unknown,
}

impl RecipeStatus {
    pub fn from_status_code(code: i32) -> Self {
        match code {
            0 => RecipeStatus::Success,
            1 => RecipeStatus::Warning,
            2 => RecipeStatus::Error,
            _ => RecipeStatus::Unknown,
        }
    }

    pub fn to_status_code(&self) -> i32 {
        match self {
            RecipeStatus::Success => 0,
            RecipeStatus::Warning => 1,
            RecipeStatus::Error => 2,
            RecipeStatus::Unknown => 3,
        }
    }
}

//  -------------------------------------------------------------
//  Run an executable, returns the recipe status
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub fn run<E, I, S>(command: S, args: I, environment: E) -> RecipeStatus
where
    E: IntoIterator<Item = (S, S)>,
    I: IntoIterator<Item = S> + Debug,
    S: AsRef<OsStr> + Display,
{
    info!("Running command {} with args {:?}", command, args);

    let result = Command::new(command)
        .args(args)
        .envs(environment)
        .output();

    match result {
        Ok(process_output) => {
            let stdout = read_bytes(&process_output.stdout);
            let stderr = read_bytes(&process_output.stderr);

            if !stdout.is_empty() {
                info!("Channel stdout: {}", stdout);
            }

            if !stderr.is_empty() {
                warn!("Channel stderr: {}", stderr);
            }

            match process_output.status.code() {
                None => {
                    warn!("Process terminated by signal.");

                    RecipeStatus::Unknown
                }
                Some(code) => RecipeStatus::from_status_code(code),
            }
        }

        Err(error) => {
            error!("Process can't spawn: {:?}", error);

            RecipeStatus::Error
        }
    }
}

fn read_bytes(bytes: &Vec<u8>) -> String {
    String::from_utf8_lossy(bytes).to_string()
}
