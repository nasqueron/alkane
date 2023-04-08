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
}

impl Display for AlkaneDeployError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Alkane deploy error: {}", self.message)
    }
}

impl Error for AlkaneDeployError {}

impl AlkaneDeployError {
    pub fn new<S>(message: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            message: message.as_ref().to_string(),
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
