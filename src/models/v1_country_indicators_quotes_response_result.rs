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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1CountryIndicatorsQuotesResponseResult {
    /// Value is the value of the indicator.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    /// Change is the change in the value of the indicator.
    #[serde(rename = "change", skip_serializing_if = "Option::is_none")]
    pub change: Option<f64>,
    /// Revised is the revised value of the indicator.
    #[serde(rename = "revised", skip_serializing_if = "Option::is_none")]
    pub revised: Option<f64>,
    /// Forecast is the forecast value of the indicator.
    #[serde(rename = "forecast", skip_serializing_if = "Option::is_none")]
    pub forecast: Option<f64>,
    /// Timestamp (in Unix epoch seconds) when this resource was initially created in the system.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

impl V1CountryIndicatorsQuotesResponseResult {
    pub fn new() -> V1CountryIndicatorsQuotesResponseResult {
        V1CountryIndicatorsQuotesResponseResult {
            value: None,
            change: None,
            revised: None,
            forecast: None,
            created_at: None,
        }
    }
}

