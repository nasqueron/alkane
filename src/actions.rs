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

pub fn initialize(
    site_name: &str,
    context: Option<String>,
    config: &AlkaneConfig,
) -> Result<RecipeStatus, DeployError> {
    let db = Database::from_config(config).ok_or_else(|| {
        let error = AlkaneDeployError::new("Can't initialize database");
        DeployError::Alkane(error)
    })?;

    let recipes = RecipesStore::from_config(config).ok_or_else(|| {
        let error = AlkaneDeployError::new("Can't initialize recipes store");
        DeployError::Alkane(error)
    })?;

    let site = config
        .get_site(site_name, context)
        .expect("Can't get site path.");
    let status = recipes.run_recipe(&site, "init");
    db.set_initialized(&site.name);

    Ok(status)
}

pub fn update(
    site_name: &str,
    context: Option<String>,
    config: &AlkaneConfig,
) -> Result<RecipeStatus, DeployError> {
    let recipes = RecipesStore::from_config(config).ok_or_else(|| {
        let error = AlkaneDeployError::new("Can't initialize recipes store");
        DeployError::Alkane(error)
    })?;

    let site = config
        .get_site(site_name, context)
        .expect("Can't get site path.");
    let status = recipes.run_recipe(&site, "update");
    Ok(status)
}

pub fn deploy(
    site_name: &str,
    context: Option<String>,
    config: &AlkaneConfig,
) -> Result<RecipeStatus, DeployError> {
    if is_present(site_name, config) {
        update(site_name, context, config)
    } else {
        initialize(site_name, context, config)
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
