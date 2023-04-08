//  -------------------------------------------------------------
//  Alkane :: Configuration
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
//  Project:        Nasqueron
//  License:        BSD-2-Clause
//  -------------------------------------------------------------

use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, MAIN_SEPARATOR_STR};

use lazy_static::lazy_static;
use log::info;
use serde::Deserialize;
use serde_yaml;

use crate::runner::site::Site;
use crate::services::tld::extract_domain_parts;

//  -------------------------------------------------------------
//  Constants:
//  - ROOTS: default path for root directories
//  - CONFIG_PATHS: paths where to find the configuration file
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

lazy_static! {
    static ref ROOTS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("db", "/var/db/alkane");
        map.insert("sites", "/var/wwwroot");
        map.insert("recipes", "/usr/local/libexec/alkane");
        map
    };
    static ref CONFIG_PATHS: Vec<&'static str> = vec![
        ".alkane.conf",
        "/etc/alkane.conf",
        "/usr/local/etc/alkane.conf",
    ];
}

//  -------------------------------------------------------------
//  AlkaneConfig is the deserialized representation of
//  the Alkane configuration file alkane.conf
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Deserialize)]
pub struct AlkaneConfig {
    /// The paths to the root directories used by Alkane
    roots: HashMap<String, String>,

    /// The template for a site directory
    site_directory_template: String,
}

#[derive(Debug)]
pub enum AlkaneConfigError {
    IO(std::io::Error),
    YAML(serde_yaml::Error),
    FileNotFound,
}

impl AlkaneConfig {
    pub fn load() -> Result<Self, AlkaneConfigError> {
        match Self::find() {
            None => Err(AlkaneConfigError::FileNotFound),
            Some(path) => {
                info!("Configuration file found: {}", &path);

                let file = File::open(&path).map_err(AlkaneConfigError::IO)?;

                serde_yaml::from_reader(file).map_err(AlkaneConfigError::YAML)
            }
        }
    }

    fn find() -> Option<String> {
        CONFIG_PATHS
            .iter()
            .filter(|&path| Path::new(path).exists())
            .map(|&path| String::from(path))
            .next()
    }

    pub fn get_root(&self, key: &str) -> Option<String> {
        if self.roots.contains_key(key) {
            self.roots.get(key).map(|s| String::from(s))
        } else {
            ROOTS.get(key).map(|s| String::from(*s))
        }
    }

    pub fn get_site(&self, site_name: &str, context: Option<String>) -> Option<Site> {
        self.get_site_path(site_name).map(|path| Site {
            name: site_name.to_string(),
            context,
            path,
        })
    }

    pub fn get_site_path(&self, site_name: &str) -> Option<String> {
        let root = self.get_root("sites")?;
        let root = root.replace("/", MAIN_SEPARATOR_STR);

        let subdir = self.resolve_site_subdir(site_name)?;

        Path::new(&root)
            .join(&subdir)
            .to_str()
            .map(|path| String::from(path))
    }

    fn resolve_site_subdir(&self, site_name: &str) -> Option<String> {
        let subdir = self
            .site_directory_template
            .clone()
            .replace("/", MAIN_SEPARATOR_STR)
            .replace("%fqdn%", site_name);

        if contains_domain_parts_variables(&subdir) {
            extract_domain_parts(site_name).map(|parts| {
                subdir
                    .replace("%subdomain%", &parts.0)
                    .replace("%domain%", &parts.1)
                    .replace("%tld%", &parts.2)
            })
        } else {
            Some(subdir)
        }
    }
}

//  -------------------------------------------------------------
//  Helper methods to extract domain name parts
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

fn contains_domain_parts_variables<S>(site_name: S) -> bool
where
    S: AsRef<str>,
{
    let site_name = site_name.as_ref();

    site_name.contains("%subdomain%")
        || site_name.contains("%domain%")
        || site_name.contains("%tld%")
}

//  -------------------------------------------------------------
//  Tests
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_load() {
        let config = AlkaneConfig::load();

        assert!(config.is_ok());
        let config = config.unwrap();

        assert_eq!(
            &config.site_directory_template,
            "%domain%.%tld%/%subdomain%"
        );
    }

    #[test]
    pub fn test_root() {
        let config = AlkaneConfig::load().unwrap();

        assert_eq!(None, config.get_root("notexisting"));
        assert_eq!(Some(String::from("tests/data/db")), config.get_root("db"));
    }

    #[test]
    pub fn test_get_site_path() {
        let config = AlkaneConfig::load().unwrap();

        let expected = Path::new("tests")
            .join("data")
            .join("wwwroot")
            .join("example.org")
            .join("foo");
        let expected = String::from(expected.to_str().unwrap());
        let expected = Some(expected);

        assert_eq!(expected, config.get_site_path("foo.example.org"));
    }

    #[test]
    pub fn test_contains_domain_parts_variables() {
        assert_eq!(
            true,
            contains_domain_parts_variables("%domain%/%subdomain%")
        );
        assert_eq!(false, contains_domain_parts_variables("%fqdn%"));
        assert_eq!(false, contains_domain_parts_variables(""));
    }
}
