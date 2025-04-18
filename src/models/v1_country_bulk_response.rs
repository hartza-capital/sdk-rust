/*
 * API Hartza Capital
 *
 * ## Welcome to the Hartza Capital API documentation  This comprehensive financial data API provides access to market information, portfolio management capabilities, and sophisticated trading tools.  For production use, an OAuth2 token is required. After authentication, you'll receive a token with a 15-minute lifespan.  When this period expires, please use your refresh token to obtain a new access token. 
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@hartza.capital
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// V1CountryBulkResponse : Countries
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1CountryBulkResponse {
    /// Ticker is the unique identifier for the country. ISO 3166-1 alpha-2 code.
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Name is the name of the country. Ex: United States.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Region is the region where the country is located. Ex: West Europe.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Continent is the continent where the country is located. Ex: Europe.
    #[serde(rename = "continent", skip_serializing_if = "Option::is_none")]
    pub continent: Option<String>,
}

impl V1CountryBulkResponse {
    /// Countries
    pub fn new() -> V1CountryBulkResponse {
        V1CountryBulkResponse {
            ticker: None,
            name: None,
            region: None,
            continent: None,
        }
    }
}

