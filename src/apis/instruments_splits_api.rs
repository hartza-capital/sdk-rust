/*
 * API Hartza Capital
 *
 * ## Welcome to the Hartza Capital API documentation  This comprehensive financial data API provides access to market information, portfolio management capabilities, and sophisticated trading tools.  For production use, an OAuth2 token is required. After authentication, you'll receive a token with a 15-minute lifespan.  When this period expires, please use your refresh token to obtain a new access token. 
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@hartza.capital
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`last_split`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LastSplitError {
    Status400(models::Orders400Response),
    Status401(models::Orders401Response),
    Status404(models::Account404Response),
    Status500(models::Orders500Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`options_last_split`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OptionsLastSplitError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`options_splits`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OptionsSplitsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_splits`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchSplitsError {
    Status400(models::Orders400Response),
    Status401(models::Orders401Response),
    Status404(models::Account404Response),
    Status500(models::Orders500Response),
    UnknownValue(serde_json::Value),
}


/// Permits to get the last split received by the shareholder for the specific instrument.
pub async fn last_split(configuration: &configuration::Configuration, ticker: &str) -> Result<models::V1SplitResponse, Error<LastSplitError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ticker = ticker;

    let uri_str = format!("{}/v1/split/{ticker}/last", configuration.base_path, ticker=crate::apis::urlencode(p_ticker));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::V1SplitResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::V1SplitResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<LastSplitError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Options method is used to describe the communication options for the targeted resource.
pub async fn options_last_split(configuration: &configuration::Configuration, ticker: &str) -> Result<(), Error<OptionsLastSplitError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ticker = ticker;

    let uri_str = format!("{}/v1/split/{ticker}/last", configuration.base_path, ticker=crate::apis::urlencode(p_ticker));
    let mut req_builder = configuration.client.request(reqwest::Method::OPTIONS, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<OptionsLastSplitError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Options method is used to describe the communication options for the targeted resource.
pub async fn options_splits(configuration: &configuration::Configuration, ) -> Result<(), Error<OptionsSplitsError>> {

    let uri_str = format!("{}/v1/splits", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::OPTIONS, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<OptionsSplitsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Permits to search splits by interval and arguments
pub async fn search_splits(configuration: &configuration::Configuration, v1_screener_np_request: models::V1ScreenerNpRequest) -> Result<models::SearchSplits200Response, Error<SearchSplitsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_v1_screener_np_request = v1_screener_np_request;

    let uri_str = format!("{}/v1/splits", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_v1_screener_np_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SearchSplits200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SearchSplits200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SearchSplitsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

