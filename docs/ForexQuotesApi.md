# \ForexQuotesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**last_quote_forex_by_ticker**](ForexQuotesApi.md#last_quote_forex_by_ticker) | **GET** /v1/eod/forex/{ticker}/last | Get Last Quote for Forex
[**lasts_quotes_forex_by_tickers**](ForexQuotesApi.md#lasts_quotes_forex_by_tickers) | **GET** /v1/eod/forex/lasts | Get Lasts Forex Quotes
[**options_last_quote_forex_by_ticker**](ForexQuotesApi.md#options_last_quote_forex_by_ticker) | **OPTIONS** /v1/eod/forex/{ticker}/last | Options Last Quote for Forex
[**options_lasts_forex_quotes**](ForexQuotesApi.md#options_lasts_forex_quotes) | **OPTIONS** /v1/eod/forex/lasts | Options Lasts Forex Quotes
[**options_search_forex_quotes**](ForexQuotesApi.md#options_search_forex_quotes) | **OPTIONS** /v1/eod/forex | Options Search Forex Quotes
[**search_quotes_forex_by_tickers**](ForexQuotesApi.md#search_quotes_forex_by_tickers) | **POST** /v1/eod/forex | Search Quotes Forex with queries or filters



## last_quote_forex_by_ticker

> models::V1QuoteResponse last_quote_forex_by_ticker(ticker)
Get Last Quote for Forex

This endpoint return the last quote received by the shareholder for the targeted Forex. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of currency exchanged | [required] |

### Return type

[**models::V1QuoteResponse**](v1QuoteResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lasts_quotes_forex_by_tickers

> models::LastsQuotesInstruments200Response lasts_quotes_forex_by_tickers(tickers)
Get Lasts Forex Quotes

This endpoint return the lasts quotes received by the shareholder for the targeted Forex. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tickers** | Option<**String**> | Some Description |  |

### Return type

[**models::LastsQuotesInstruments200Response**](LastsQuotesInstruments_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_last_quote_forex_by_ticker

> options_last_quote_forex_by_ticker(ticker)
Options Last Quote for Forex

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of currency exchanged | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_lasts_forex_quotes

> options_lasts_forex_quotes()
Options Lasts Forex Quotes

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


## options_search_forex_quotes

> options_search_forex_quotes()
Options Search Forex Quotes

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


## search_quotes_forex_by_tickers

> models::SearchQuotesPortfolios200Response search_quotes_forex_by_tickers(v1_screener_interval_request)
Search Quotes Forex with queries or filters

This endpoint allow to use search with complexe queries (keywords, filters, sort etc..) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_interval_request** | [**V1ScreenerIntervalRequest**](V1ScreenerIntervalRequest.md) | Body of the request to search quotes for Forex | [required] |

### Return type

[**models::SearchQuotesPortfolios200Response**](SearchQuotesPortfolios_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

