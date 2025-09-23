//  -------------------------------------------------------------
//  Alkane :: Server :: Requests
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
//  Project:        Nasqueron
//  License:        BSD-2-Clause
//  -------------------------------------------------------------

use axum::extract::{Path, State};
use axum::http::StatusCode;

use limiting_factor_axum::api::guards::AxumRequestBody as RequestBody;
use limiting_factor_axum::api::replies::{ApiJsonResponse, ApiResponse, FailureResponse};

use log::{debug, info, warn};
use crate::actions;
use crate::config::AlkaneConfig;
use crate::deploy::DeployError;
use crate::runner::RecipeStatus;

//  -------------------------------------------------------------
//  Monitoring
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub async fn status() -> &'static str {
    "ALIVE"
}

//  -------------------------------------------------------------
//  Alkane requests
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub async fn is_present(
    Path(site_name): Path<String>,
    State(config): State<AlkaneConfig>,
) -> ApiJsonResponse<bool> {
    actions::is_present(&site_name, &config).into_json_response()
}

pub async fn init(
    Path(site_name): Path<String>,
    State(config): State<AlkaneConfig>,
    context: RequestBody,
) -> ApiJsonResponse<RecipeStatus> {
    info!("Deploying {}", &site_name);

    let context = context.into_optional_string();
    debug!("Context: {:?}", &context);

    actions::initialize(&site_name, context, &config)
        .into_json_response()
}

pub async fn update(
    Path(site_name): Path<String>,
    State(config): State<AlkaneConfig>,
    context: RequestBody,
) -> ApiJsonResponse<RecipeStatus> {
    info!("Deploying {}", &site_name);

    let context = context.into_optional_string();
    debug!("Context: {:?}", &context);

    actions::update(&site_name, context, &config)
        .into_json_response()
}

pub async fn deploy(
    Path(site_name): Path<String>,
    State(config): State<AlkaneConfig>,
    context: RequestBody,
) -> ApiJsonResponse<RecipeStatus> {
    info!("Deploying {}", &site_name);

    let context = context.into_optional_string();
    debug!("Context: {:?}", &context);

    actions::deploy(&site_name, context, &config)
        .into_json_response()
}

//  -------------------------------------------------------------
//  Custom error handling
//
//  Deploy errors are returned as 400 + the Alkane error message
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl FailureResponse for DeployError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn response(&self) -> String {
        warn!("{}", self);     // Server log
        format!("{}", self)    // API response
    }
}
