//  -------------------------------------------------------------
//  Alkane :: Runner :: Recipes store
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
//  Project:        Nasqueron
//  License:        BSD-2-Clause
//  -------------------------------------------------------------

use std::collections::HashMap;
use std::path::Path;

use crate::config::AlkaneConfig;
use crate::runner::run;
use crate::runner::site::Site;
use crate::runner::RecipeStatus;

pub struct RecipesStore {
    root: String,
}

impl RecipesStore {
    pub fn new<S>(root: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            root: root.as_ref().to_string(),
        }
    }

    pub fn from_config(config: &AlkaneConfig) -> Option<Self> {
        config.get_root("recipes").map(Self::new)
    }

    fn get_recipe_path(&self, site_name: &str, action: &str) -> String {
        Path::new(&self.root)
            .join(site_name)
            .join(action)
            .to_str()
            .expect("Can't read recipe path as UTF-8")
            .to_string()
    }

    pub fn run_recipe(&self, site: &Site, action: &str) -> RecipeStatus {
        let command = self.get_recipe_path(&site.name, action);
        let environment = self.get_environment(&site);

        run(command, Vec::new(), environment)
    }

    fn get_environment(&self, site: &Site) -> HashMap<String, String> {
        let mut map = HashMap::new();

        map.insert("ALKANE_RECIPES_PATH".to_string(), self.root.clone());
        map.insert("ALKANE_SITE_NAME".to_string(), site.name.clone());
        map.insert("ALKANE_SITE_PATH".to_string(), site.path.clone());

        if let Some(context) = &site.context {
            map.insert("ALKANE_SITE_CONTEXT".to_string(), context.clone());
        }

        map
    }
}

//  -------------------------------------------------------------
//  Tests
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[cfg(test)]
mod tests {
    use std::path::MAIN_SEPARATOR_STR;

    use super::*;

    #[test]
    pub fn test_get_recipe_path() {
        let expected = "tests/data/recipes/foo.acme.tld/update".replace("/", MAIN_SEPARATOR_STR);

        let test_store_path = "tests/data/recipes".replace("/", MAIN_SEPARATOR_STR);
        let store = RecipesStore::new(&test_store_path);

        assert_eq!(expected, store.get_recipe_path("foo.acme.tld", "update"));
    }

    #[test]
    pub fn test_get_environment() {
        let mut expected = HashMap::new();
        expected.insert(
            "ALKANE_RECIPES_PATH".to_string(),
            "tests/data/recipes".replace("/", MAIN_SEPARATOR_STR),
        );
        expected.insert("ALKANE_SITE_NAME".to_string(), "foo.acme.tld".to_string());
        expected.insert(
            "ALKANE_SITE_PATH".to_string(),
            "tests/data/wwwroot/acme.tld/foo".replace("/", MAIN_SEPARATOR_STR),
        );

        let test_store_path = "tests/data/recipes".replace("/", MAIN_SEPARATOR_STR);
        let store = RecipesStore::new(&test_store_path);

        let site = Site {
            name: "foo.acme.tld".to_string(),
            path: "tests/data/wwwroot/acme.tld/foo".replace("/", MAIN_SEPARATOR_STR),
            context: None,
        };

        assert_eq!(expected, store.get_environment(&site));
    }

    #[test]
    pub fn test_get_environment_with_context() {
        let site = Site {
            name: "foo.acme.tld".to_string(),
            path: "/path/to/site".to_string(),
            context: Some("CH3-CH3".to_string()),
        };

        let environment = RecipesStore::new("/path/to/recipes").get_environment(&site);

        assert!(environment.contains_key("ALKANE_SITE_CONTEXT"));
        assert_eq!("CH3-CH3", environment["ALKANE_SITE_CONTEXT"])
    }
}
