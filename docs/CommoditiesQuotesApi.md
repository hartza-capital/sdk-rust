# \CommoditiesQuotesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**last_quote_commodity_by_ticker**](CommoditiesQuotesApi.md#last_quote_commodity_by_ticker) | **GET** /v1/eod/commodity/{ticker}/last | Last Quote for Instrument
[**lasts_quotes_commodities**](CommoditiesQuotesApi.md#lasts_quotes_commodities) | **GET** /v1/eod/commodities/lasts | Lasts Commodity Quotes
[**options_last_quote_commodity_by_ticker**](CommoditiesQuotesApi.md#options_last_quote_commodity_by_ticker) | **OPTIONS** /v1/eod/commodity/{ticker}/last | Options Last Quote for Commodity
[**options_lasts_quotes_commodities**](CommoditiesQuotesApi.md#options_lasts_quotes_commodities) | **OPTIONS** /v1/eod/commodities/lasts | Options Lasts Commodity Quotes
[**options_quotes_commodity_histogram**](CommoditiesQuotesApi.md#options_quotes_commodity_histogram) | **OPTIONS** /v1/eod/commodities/histogram | Options Commodities Quotes Histogram
[**search_quotes_commodity_histogram**](CommoditiesQuotesApi.md#search_quotes_commodity_histogram) | **POST** /v1/eod/commodities/histogram | Search Commodity Quotes Histogram



## last_quote_commodity_by_ticker

> models::V1QuoteResponse last_quote_commodity_by_ticker(ticker)
Last Quote for Instrument

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of commodity | [required] |

### Return type

[**models::V1QuoteResponse**](v1QuoteResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lasts_quotes_commodities

> models::LastsQuotesInstruments200Response lasts_quotes_commodities(tickers)
Lasts Commodity Quotes

This endpoint return the lasts quotes received by the shareholder for the targeted Commodities. 

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


## options_last_quote_commodity_by_ticker

> options_last_quote_commodity_by_ticker(ticker)
Options Last Quote for Commodity

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of commodity | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_lasts_quotes_commodities

> options_lasts_quotes_commodities()
Options Lasts Commodity Quotes

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


## options_quotes_commodity_histogram

> options_quotes_commodity_histogram()
Options Commodities Quotes Histogram

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


## search_quotes_commodity_histogram

> models::SearchQuotesPortfolioHistogram200Response search_quotes_commodity_histogram(v1_screener_interval_request)
Search Commodity Quotes Histogram

This endpoint allow to use search with complexe queries (keywords, filters, sort etc..) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_interval_request** | [**V1ScreenerIntervalRequest**](V1ScreenerIntervalRequest.md) | Body of the request to search quotes for Commodities | [required] |

### Return type

[**models::SearchQuotesPortfolioHistogram200Response**](SearchQuotesPortfolioHistogram_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

