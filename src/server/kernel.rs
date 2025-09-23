//  -------------------------------------------------------------
//  Alkane :: Server :: Kernel
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
//  Project:        Nasqueron
//  License:        BSD-2-Clause
//  -------------------------------------------------------------

use axum::Router;
use axum::routing::{get, post};
use limiting_factor_axum::app::{App, ServerConfig};

use crate::config::AlkaneConfig;
use crate::server::requests::*;

//  -------------------------------------------------------------
//  Server entry point
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub fn get_default_config () -> ServerConfig {
    ServerConfig {
        address: "0.0.0.0".to_string(),
        port: 8000,
        mount_point: "/".to_string(),
    }
}

pub fn get_router () -> Router<AlkaneConfig> {
    Router::new()

        // Monitoring
        .route("/status", get(status))

        // Alkane API
        .route("/init/{site_name}", post(init))
        .route("/update/{site_name}", post(update))
        .route("/deploy/{site_name}", post(deploy))
        .route("/is_present/{site_name}", get(is_present))
}

pub async fn run(alkane_config: AlkaneConfig) -> bool {
    let server_config = ServerConfig::from_env_or(get_default_config());

    let router = get_router()
        .with_state(alkane_config);

    App::new(server_config, router).run().await
}
