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


/// struct for typed errors of method [`account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountError {
    Status400(models::Orders400Response),
    Status401(models::Orders401Response),
    Status404(models::Account404Response),
    Status500(models::Orders500Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`accounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountsError {
    Status401(models::Orders401Response),
    Status404(models::Account404Response),
    Status500(models::Orders500Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`archive_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArchiveAccountError {
    Status400(models::Orders400Response),
    Status401(models::Orders401Response),
    Status404(models::Account404Response),
    Status500(models::Orders500Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAccountError {
    Status400(models::Orders400Response),
    Status401(models::Orders401Response),
    Status409(models::CreateAccount409Response),
    Status500(models::Orders500Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`options_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OptionsAccountError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`options_accounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OptionsAccountsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`patch_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchAccountError {
    Status400(models::Orders400Response),
    Status401(models::Orders401Response),
    Status404(models::Account404Response),
    Status500(models::Orders500Response),
    UnknownValue(serde_json::Value),
}


/// Permits to get Account properties, watchlists and strategy applicated on the portfolios
pub async fn account(configuration: &configuration::Configuration, x_account: &str) -> Result<models::Account200Response, Error<AccountError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_x_account = x_account;

    let uri_str = format!("{}/v1/account", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("X-Account", p_x_account.to_string());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Account200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Account200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AccountError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Permits to list positions, cash and value of the portfolios
pub async fn accounts(configuration: &configuration::Configuration, items: i64, page: i64) -> Result<models::Accounts200Response, Error<AccountsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_items = items;
    let p_page = page;

    let uri_str = format!("{}/v1/accounts", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("items", &p_items.to_string())]);
    req_builder = req_builder.query(&[("page", &p_page.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Accounts200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Accounts200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AccountsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Permits to archive Account and disable strategy applicated on the portfolios
pub async fn archive_account(configuration: &configuration::Configuration, x_account: &str) -> Result<models::Account200Response, Error<ArchiveAccountError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_x_account = x_account;

    let uri_str = format!("{}/v1/account", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("X-Account", p_x_account.to_string());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Account200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Account200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ArchiveAccountError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Create Account permit to create a new account with strategy and properties.
pub async fn create_account(configuration: &configuration::Configuration, create_account_request: Option<models::CreateAccountRequest>) -> Result<models::Account200Response, Error<CreateAccountError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_create_account_request = create_account_request;

    let uri_str = format!("{}/v1/accounts", configuration.base_path);
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
    req_builder = req_builder.json(&p_create_account_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Account200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Account200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateAccountError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Options method is used to describe the communication options for the targeted resource.
pub async fn options_account(configuration: &configuration::Configuration, ) -> Result<(), Error<OptionsAccountError>> {

    let uri_str = format!("{}/v1/account", configuration.base_path);
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
        let entity: Option<OptionsAccountError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Options method is used to describe the communication options for the targeted resource.
pub async fn options_accounts(configuration: &configuration::Configuration, ) -> Result<(), Error<OptionsAccountsError>> {

    let uri_str = format!("{}/v1/accounts", configuration.base_path);
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
        let entity: Option<OptionsAccountsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Permits to patch Account properties, watchlists and strategy applicated on the portfolios
pub async fn patch_account(configuration: &configuration::Configuration, x_account: &str, patch_account_request: Option<models::PatchAccountRequest>) -> Result<models::Account200Response, Error<PatchAccountError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_x_account = x_account;
    let p_patch_account_request = patch_account_request;

    let uri_str = format!("{}/v1/account", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("X-Account", p_x_account.to_string());
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_patch_account_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Account200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Account200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PatchAccountError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

