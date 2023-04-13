//  -------------------------------------------------------------
//  Alkane :: Deploy
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
//  Project:        Nasqueron
//  License:        BSD-2-Clause
//  -------------------------------------------------------------

use std::error::Error;
use std::fmt::{Display, Formatter};

//  -------------------------------------------------------------
//  Errors during our own workflow deployment
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

/// Represents an error during the workflow to run a deployment
#[derive(Debug)]
pub struct AlkaneDeployError {
    pub message: String,

    /// The name of the site to deploy
    pub site_name: String,

    /// The deployment action, "init" or "update"
    pub action: String,
}

impl Display for AlkaneDeployError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Can't run deployment action '{}' for site '{}': {}", self.action, self.site_name, self.message)
    }
}

impl Error for AlkaneDeployError {}

impl AlkaneDeployError {
    pub fn new<S>(message: S, site_name: S, action: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            message: message.as_ref().to_string(),
            site_name: site_name.as_ref().to_string(),
            action: action.as_ref().to_string(),
        }
    }
}

//  -------------------------------------------------------------
//  Errors that can occur during a deployment
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug)]
pub enum DeployError {
    Alkane(AlkaneDeployError),
}

impl Display for DeployError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeployError::Alkane(error) => error.fmt(f),
        }
    }
}
