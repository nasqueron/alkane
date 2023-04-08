//  -------------------------------------------------------------
//  Alkane :: Commands
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
//  Project:        Nasqueron
//  License:        BSD-2-Clause
//  -------------------------------------------------------------

use clap::{Args, Parser};

//  -------------------------------------------------------------
//  Main command
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Parser)]
#[command(name = "alkane")]
#[clap(author="Nasqueron project", version, about="Manage Alkane PaaS", long_about=None)]
pub enum AlkaneCommand {
    /// Launch an HTTP server to expose the Alkane REST API
    Server(ServerArgs),

    /// Initialize a site
    #[command(arg_required_else_help = true)]
    Init(DeployArgs),

    /// Update site assets to latest version
    #[command(arg_required_else_help = true)]
    Update(DeployArgs),

    /// Initialize of if already initialized update a site
    #[command(arg_required_else_help = true)]
    Deploy(DeployArgs),

    /// Determine if a domain is served on our PaaS
    #[command(name = "is-present", arg_required_else_help = true)]
    IsPresent(IsPresentArgs),
}

//  -------------------------------------------------------------
//   Subcommands arguments
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Args)]
pub struct ServerArgs {
    #[arg(long, default_value = "/")]
    pub mounting_point: String,
}

#[derive(Debug, Args)]
pub struct DeployArgs {
    /// The name of the site to deploy, using sub.domain.tld format
    pub site_name: String,

    /// The artifact to deploy. Allows CD to give metadata or an URL to download last artifact
    pub artifact: Option<String>,
}

#[derive(Debug, Args)]
pub struct IsPresentArgs {
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,

    /// The name of the site to deploy, using sub.domain.tld format
    pub site_name: String,
}

//  -------------------------------------------------------------
//  Helper methods
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub trait ToStatusCode {
    fn to_status_code(&self) -> i32;
}

impl ToStatusCode for bool {
    fn to_status_code(&self) -> i32 {
        if *self {
            0
        } else {
            1
        }
    }
}

impl<T> ToStatusCode for Option<T> {
    fn to_status_code(&self) -> i32 {
        self.is_some().to_status_code()
    }
}

impl<T, E> ToStatusCode for Result<T, E> {
    fn to_status_code(&self) -> i32 {
        self.is_ok().to_status_code()
    }
}
