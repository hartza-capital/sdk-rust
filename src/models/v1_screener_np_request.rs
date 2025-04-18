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

/// V1ScreenerNpRequest : v1ScreenerNPRequest is used to sepecify your search without pagination.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1ScreenerNpRequest {
    /// Filters is a list of filters.
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<models::V1ScreenerFilter>>,
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<Box<models::V1ScreenerSort>>,
}

impl V1ScreenerNpRequest {
    /// v1ScreenerNPRequest is used to sepecify your search without pagination.
    pub fn new() -> V1ScreenerNpRequest {
        V1ScreenerNpRequest {
            filters: None,
            sort: None,
        }
    }
}

