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
pub struct V1SearchResponse {
    /// Id is the unique identifier for the instrument.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Unique identifier symbol for this financial instrument, following standard market conventions.
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// TickerAlternative is an alternative identifier for the instrument.
    #[serde(rename = "ticker_alternative", skip_serializing_if = "Option::is_none")]
    pub ticker_alternative: Option<String>,
    /// Exchange is the exchange where the instrument is traded.
    #[serde(rename = "exchange", skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    /// Name is the name of the instrument.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ISIN is the International Securities Identification Number.
    #[serde(rename = "isin", skip_serializing_if = "Option::is_none")]
    pub isin: Option<String>,
    /// CIK is the Central Index Key. Only for US companies.
    #[serde(rename = "cik", skip_serializing_if = "Option::is_none")]
    pub cik: Option<String>,
    /// Asset is the asset class of the instrument.
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    /// Activity is the activity of the instrument.
    #[serde(rename = "activity", skip_serializing_if = "Option::is_none")]
    pub activity: Option<i64>,
    /// ADR is a boolean value that indicates if the instrument is an American Depositary Receipt.
    #[serde(rename = "adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<bool>,
    /// Activities is a list of activities that the instrument is involved in.
    #[serde(rename = "activities", skip_serializing_if = "Option::is_none")]
    pub activities: Option<Vec<String>>,
    /// Size is the size of the instrument.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// History is a list of historical quotes for the instrument. The list is sorted by date in ascending order.
    #[serde(rename = "history", skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<f32>>,
    /// Change is the change in the price of the instrument.
    #[serde(rename = "change", skip_serializing_if = "Option::is_none")]
    pub change: Option<f64>,
    /// Last is the last price of the instrument.
    #[serde(rename = "last", skip_serializing_if = "Option::is_none")]
    pub last: Option<f64>,
    /// Start is the start price of the instrument.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
    /// MaxAnnual is the maximum annual price of the instrument.
    #[serde(rename = "max_annual", skip_serializing_if = "Option::is_none")]
    pub max_annual: Option<f64>,
    /// MinAnnual is the minimum annual price of the instrument.
    #[serde(rename = "min_annual", skip_serializing_if = "Option::is_none")]
    pub min_annual: Option<f64>,
    /// Currency is the currency of the instrument.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Country is the country of the instrument.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// VolumeAvg10d is the average volume of the instrument over the last 10 days.
    #[serde(rename = "volume_avg_10d", skip_serializing_if = "Option::is_none")]
    pub volume_avg_10d: Option<i64>,
    /// VolumeAvg30d is the average volume of the instrument over the last 30 days.
    #[serde(rename = "volume_avg_30d", skip_serializing_if = "Option::is_none")]
    pub volume_avg_30d: Option<i64>,
    /// VolumeAvg90d is the average volume of the instrument over the last 90 days.
    #[serde(rename = "volume_avg_90d", skip_serializing_if = "Option::is_none")]
    pub volume_avg_90d: Option<i64>,
    /// VolumeRatio is the ratio of the volume of the instrument to the average volume over the last 90 days.
    #[serde(rename = "volume_ratio", skip_serializing_if = "Option::is_none")]
    pub volume_ratio: Option<f64>,
    /// MarketCapUSD is the market capitalization of the instrument in USD.
    #[serde(rename = "market_cap_usd", skip_serializing_if = "Option::is_none")]
    pub market_cap_usd: Option<i64>,
    /// ReturnYear is the return of the instrument over the last year.
    #[serde(rename = "return_year", skip_serializing_if = "Option::is_none")]
    pub return_year: Option<f64>,
    /// DividendYield is the dividend yield of the instrument.
    #[serde(rename = "dividend_yield", skip_serializing_if = "Option::is_none")]
    pub dividend_yield: Option<f64>,
    /// Beta5y is the beta of the instrument over the last 5 years.
    #[serde(rename = "beta5y", skip_serializing_if = "Option::is_none")]
    pub beta5y: Option<f64>,
    /// Timestamp (in Unix epoch seconds) when this resource was initially created in the system.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// Timestamp (in Unix epoch seconds) when this resource was last modified.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

impl V1SearchResponse {
    pub fn new() -> V1SearchResponse {
        V1SearchResponse {
            id: None,
            ticker: None,
            ticker_alternative: None,
            exchange: None,
            name: None,
            isin: None,
            cik: None,
            asset: None,
            activity: None,
            adr: None,
            activities: None,
            size: None,
            history: None,
            change: None,
            last: None,
            start: None,
            max_annual: None,
            min_annual: None,
            currency: None,
            country: None,
            volume_avg_10d: None,
            volume_avg_30d: None,
            volume_avg_90d: None,
            volume_ratio: None,
            market_cap_usd: None,
            return_year: None,
            dividend_yield: None,
            beta5y: None,
            created_at: None,
            updated_at: None,
        }
    }
}

