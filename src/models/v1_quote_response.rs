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
pub struct V1QuoteResponse {
    /// Change is the change in the price of the security.
    #[serde(rename = "change", skip_serializing_if = "Option::is_none")]
    pub change: Option<f64>,
    /// Performance is the performance of the security.
    #[serde(rename = "performance", skip_serializing_if = "Option::is_none")]
    pub performance: Option<f64>,
    /// MarketCap is the market capitalization of the security.
    #[serde(rename = "market_cap", skip_serializing_if = "Option::is_none")]
    pub market_cap: Option<f64>,
    /// Open is the opening price of the security.
    #[serde(rename = "open", skip_serializing_if = "Option::is_none")]
    pub open: Option<f64>,
    /// High is the high price of the security.
    #[serde(rename = "high", skip_serializing_if = "Option::is_none")]
    pub high: Option<f64>,
    /// Low is the low price of the security.
    #[serde(rename = "low", skip_serializing_if = "Option::is_none")]
    pub low: Option<f64>,
    /// Close is the closing price of the security.
    #[serde(rename = "close", skip_serializing_if = "Option::is_none")]
    pub close: Option<f64>,
    /// Volume is the volume of the security.
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<i64>,
    /// Timestamp (in Unix epoch seconds) when this resource was initially created in the system.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

impl V1QuoteResponse {
    pub fn new() -> V1QuoteResponse {
        V1QuoteResponse {
            change: None,
            performance: None,
            market_cap: None,
            open: None,
            high: None,
            low: None,
            close: None,
            volume: None,
            created_at: None,
        }
    }
}

