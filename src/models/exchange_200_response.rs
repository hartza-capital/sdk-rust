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
pub struct Exchange200Response {
    /// Unique identifier symbol for this financial instrument, following standard market conventions.
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// TickerAlternatives is a list of alternative identifiers for the exchange.
    #[serde(rename = "ticker_alternatives", skip_serializing_if = "Option::is_none")]
    pub ticker_alternatives: Option<Vec<String>>,
    /// Name is the name of the exchange.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description is the description of the exchange.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// PrimaryIndex is the primary index of the exchange. It is the main index of the exchange.
    #[serde(rename = "primary_index", skip_serializing_if = "Option::is_none")]
    pub primary_index: Option<String>,
    /// Indexes is a list of indexes that are part of the exchange.
    #[serde(rename = "indexes", skip_serializing_if = "Option::is_none")]
    pub indexes: Option<Vec<models::V1SearchIndexsResponseResult>>,
    #[serde(rename = "stats", skip_serializing_if = "Option::is_none")]
    pub stats: Option<Box<models::Exchange200ResponseStats>>,
    /// Routing is the routing code for the exchange. It is the code used to route orders to the exchange.
    #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
    pub routing: Option<String>,
    /// Country is the country where the exchange is located. ISO 3166-1 alpha-2 code.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Currency is the currency used by the exchange.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Timezone is the timezone where the exchange is located. Ex: America/New_York.
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// SizeCaps is a list of size caps for the exchange.
    #[serde(rename = "size_caps", skip_serializing_if = "Option::is_none")]
    pub size_caps: Option<Vec<models::V1GetResponseSizeCap>>,
    /// Holidays is a list of holidays for the exchange.
    #[serde(rename = "holidays", skip_serializing_if = "Option::is_none")]
    pub holidays: Option<Vec<models::Exchange200ResponseHolidaysInner>>,
    /// MarketHours is the trading hours of the exchange.
    #[serde(rename = "market_hours", skip_serializing_if = "Option::is_none")]
    pub market_hours: Option<String>,
    /// Next close/open in seconds
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<i64>,
    /// Status is the status of the exchange.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl Exchange200Response {
    pub fn new() -> Exchange200Response {
        Exchange200Response {
            ticker: None,
            ticker_alternatives: None,
            name: None,
            description: None,
            primary_index: None,
            indexes: None,
            stats: None,
            routing: None,
            country: None,
            currency: None,
            timezone: None,
            size_caps: None,
            holidays: None,
            market_hours: None,
            next: None,
            status: None,
        }
    }
}
/// Status is the status of the exchange.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "CLOSED")]
    Closed,
    #[serde(rename = "PRE_MARKET")]
    PreMarket,
    #[serde(rename = "POST_MARKET")]
    PostMarket,
}

impl Default for Status {
    fn default() -> Status {
        Self::Open
    }
}

