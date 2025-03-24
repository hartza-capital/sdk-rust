# \PortfoliosQuotesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**lasts_quotes_portfolios**](PortfoliosQuotesApi.md#lasts_quotes_portfolios) | **GET** /v1/quotes/portfolios/lasts | List lasts quotes for Portfolios
[**options_lasts_portfolios_quotes**](PortfoliosQuotesApi.md#options_lasts_portfolios_quotes) | **OPTIONS** /v1/quotes/portfolios/lasts | Options List lasts quote for Portfolios
[**options_portfolio_last_quote**](PortfoliosQuotesApi.md#options_portfolio_last_quote) | **OPTIONS** /v1/quotes/portfolio/{ticker}/last | Options Last Quote for Account Portfolio by Ticker
[**options_portfolios_quotes**](PortfoliosQuotesApi.md#options_portfolios_quotes) | **OPTIONS** /v1/quotes/portfolios | Options Quotes by currency and period
[**portfolio_last_quote**](PortfoliosQuotesApi.md#portfolio_last_quote) | **GET** /v1/quotes/portfolio/{ticker}/last | Last Quote for Account Portfolio
[**portfolios_quotes**](PortfoliosQuotesApi.md#portfolios_quotes) | **POST** /v1/quotes/portfolios | Search Quotes by currency and period



## lasts_quotes_portfolios

> models::LastsQuotesPortfolios200Response lasts_quotes_portfolios(tickers)
List lasts quotes for Portfolios

Permits to list lasts quotes for the portfolios

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tickers** | Option<**String**> | Some Description |  |

### Return type

[**models::LastsQuotesPortfolios200Response**](LastsQuotesPortfolios_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_lasts_portfolios_quotes

> options_lasts_portfolios_quotes()
Options List lasts quote for Portfolios

Options method is used to describe the communication options for the targeted resource.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_portfolio_last_quote

> options_portfolio_last_quote(ticker)
Options Last Quote for Account Portfolio by Ticker

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of currency | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_portfolios_quotes

> options_portfolios_quotes()
Options Quotes by currency and period

Options method is used to describe the communication options for the targeted resource.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_last_quote

> models::V1QuoteResponse portfolio_last_quote(ticker)
Last Quote for Account Portfolio

Permits to get the last quote received by the shareholder for a portfolio.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of currency | [required] |

### Return type

[**models::V1QuoteResponse**](v1QuoteResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolios_quotes

> models::PortfoliosQuotes200Response portfolios_quotes(v1_screener_interval_request)
Search Quotes by currency and period

Permits to search quotes by currency and period

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_interval_request** | [**V1ScreenerIntervalRequest**](V1ScreenerIntervalRequest.md) | Body of the request to search quotes by currency and period | [required] |

### Return type

[**models::PortfoliosQuotes200Response**](PortfoliosQuotes_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

