//  -------------------------------------------------------------
//  Alkane :: Actions
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
//  Project:        Nasqueron
//  License:        BSD-2-Clause
//  -------------------------------------------------------------

use crate::command::ServerArgs;
use crate::config::AlkaneConfig;
use crate::db::Database;
use crate::deploy::AlkaneDeployError;
use crate::deploy::DeployError;
use crate::runner::store::RecipesStore;
use crate::runner::RecipeStatus;
use crate::server::kernel::run;

//  -------------------------------------------------------------
//  Actions only available in CLI
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub fn serve(args: ServerArgs, config: AlkaneConfig) {
    run(config, &args.mounting_point);
}

//  -------------------------------------------------------------
//  Actions available both for CLI and HTTP
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

fn run_deployment_action(
    site_name: &str,
    context: Option<String>,
    config: &AlkaneConfig,
    action: &str,
) -> Result<RecipeStatus, DeployError> {
    let db = Database::from_config(config).ok_or_else(|| {
        let error = AlkaneDeployError::new("Can't initialize database", site_name, action);
        DeployError::Alkane(error)
    })?;

    let recipes = RecipesStore::from_config(config).ok_or_else(|| {
        let error = AlkaneDeployError::new("Can't initialize recipes store", site_name, action);
        DeployError::Alkane(error)
    })?;

    let site = config
        .get_site(site_name, context)
        .ok_or_else(|| {
            let error = AlkaneDeployError::new("Can't resolve site path", site_name, action);
            DeployError::Alkane(error)
        })?;

    let status = recipes.run_recipe(&site, action);

    if action == "init" && status == RecipeStatus::Success {
        db.set_initialized(&site.name);
    }

    Ok(status)
}

pub fn initialize(
    site_name: &str,
    context: Option<String>,
    config: &AlkaneConfig,
) -> Result<RecipeStatus, DeployError> {
    run_deployment_action(site_name, context, config, "init")
}

pub fn update(
    site_name: &str,
    context: Option<String>,
    config: &AlkaneConfig,
) -> Result<RecipeStatus, DeployError> {
    run_deployment_action(site_name, context, config, "update")
}

pub fn deploy(
    site_name: &str,
    context: Option<String>,
    config: &AlkaneConfig,
) -> Result<RecipeStatus, DeployError> {
    if is_present(site_name, config) {
        run_deployment_action(site_name, context, config, "update")
    } else {
        run_deployment_action(site_name, context, config, "init")
    }
}

pub fn is_present(site_name: &str, config: &AlkaneConfig) -> bool {
    match Database::from_config(&config) {
        None => false,
        Some(db) => db.is_initialized(site_name),
    }
}

//  -------------------------------------------------------------
//  Tests
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_is_present() {
        let config = AlkaneConfig::load().unwrap();

        assert_eq!(true, is_present("foo.acme.tld", &config));
        assert_eq!(false, is_present("notexisting.acme.tld", &config));
    }
}
