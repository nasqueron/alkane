//  -------------------------------------------------------------
//  Alkane :: Server :: Requests
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
//  Project:        Nasqueron
//  License:        BSD-2-Clause
//  -------------------------------------------------------------

use limiting_factor::api::replies::{ApiJsonResponse, ApiResponse};
use log::{debug, info, warn};
use rocket::State;
use rocket_codegen::{get, post};

use crate::actions;
use crate::config::AlkaneConfig;

//  -------------------------------------------------------------
//  Monitoring
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[get("/status")]
pub fn status() -> &'static str {
    "ALIVE"
}

//  -------------------------------------------------------------
//  Alkane requests
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[get("/is_present/<site_name>")]
pub fn is_present(site_name: String, config: State<AlkaneConfig>) -> ApiJsonResponse<bool> {
    actions::is_present(&site_name, &config).into_json_response()
}

#[post("/init/<site_name>", data = "<context>")]
pub fn init(
    site_name: String,
    context: String,
    config: State<AlkaneConfig>,
) -> ApiJsonResponse<bool> {
    info!("Deploying {}", &site_name);

    let context = if context.is_empty() {
        None
    } else {
        Some(context)
    };
    debug!("Context: {:?}", &context);

    match actions::initialize(&site_name, context, &config) {
        Ok(_) => true.into_json_response(),
        Err(error) => {
            warn!("Deployment error: {:?}", error);

            false.into_json_response()
        }
    }
}

#[post("/update/<site_name>", data = "<context>")]
pub fn update(
    site_name: String,
    context: String,
    config: State<AlkaneConfig>,
) -> ApiJsonResponse<bool> {
    info!("Deploying {}", &site_name);

    let context = if context.is_empty() {
        None
    } else {
        Some(context)
    };
    debug!("Context: {:?}", &context);

    match actions::update(&site_name, context, &config) {
        Ok(_) => true.into_json_response(),
        Err(error) => {
            warn!("Deployment error: {:?}", error);

            false.into_json_response()
        }
    }
}

#[post("/deploy/<site_name>", data = "<context>")]
pub fn deploy(
    site_name: String,
    context: String,
    config: State<AlkaneConfig>,
) -> ApiJsonResponse<bool> {
    info!("Deploying {}", &site_name);

    let context = if context.is_empty() {
        None
    } else {
        Some(context)
    };
    debug!("Context: {:?}", &context);

    match actions::deploy(&site_name, context, &config) {
        Ok(_) => true.into_json_response(),
        Err(error) => {
            warn!("Deployment error: {:?}", error);

            false.into_json_response()
        }
    }
}
