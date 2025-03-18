# \AccountsPortfoliosCurrenciesQuotesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**last_quote_portfolio_by_ticker**](AccountsPortfoliosCurrenciesQuotesApi.md#last_quote_portfolio_by_ticker) | **GET** /v1/portfolio/{ticker}/currencies/last | Last Quote for Account Portfolio by Currency
[**options_last_quote_portfolio_by_ticker**](AccountsPortfoliosCurrenciesQuotesApi.md#options_last_quote_portfolio_by_ticker) | **OPTIONS** /v1/portfolio/{ticker}/currencies/last | Options Last Quote for Instrument
[**options_quotes_portfolio_currencies**](AccountsPortfoliosCurrenciesQuotesApi.md#options_quotes_portfolio_currencies) | **OPTIONS** /v1/portfolio/currencies | Options Quotes by currency and period
[**search_quotes_portfolio_currencies**](AccountsPortfoliosCurrenciesQuotesApi.md#search_quotes_portfolio_currencies) | **POST** /v1/portfolio/currencies | Search Quotes by currency and period



## last_quote_portfolio_by_ticker

> models::V1QuoteResponse last_quote_portfolio_by_ticker(ticker)
Last Quote for Account Portfolio by Currency

Last Quote permit to get the last quote for a currency

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


## options_last_quote_portfolio_by_ticker

> options_last_quote_portfolio_by_ticker(ticker)
Options Last Quote for Instrument

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


## options_quotes_portfolio_currencies

> options_quotes_portfolio_currencies()
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


## search_quotes_portfolio_currencies

> models::SearchQuotesPortfolios200Response search_quotes_portfolio_currencies(v1_screener_interval_request)
Search Quotes by currency and period

Search Quotes permit to search quotes for a currency

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_interval_request** | [**V1ScreenerIntervalRequest**](V1ScreenerIntervalRequest.md) | Body of the request to search quotes by currency and period | [required] |

### Return type

[**models::SearchQuotesPortfolios200Response**](SearchQuotesPortfolios_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

