# \ForexHistogramApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**options_search_quotes_forex_histogram**](ForexHistogramApi.md#options_search_quotes_forex_histogram) | **OPTIONS** /v1/eod/forex/histogram | Options Search Forex Quotes Histogram
[**search_quotes_forex_histogram**](ForexHistogramApi.md#search_quotes_forex_histogram) | **POST** /v1/eod/forex/histogram | Search Quotes Histogram



## options_search_quotes_forex_histogram

> options_search_quotes_forex_histogram()
Options Search Forex Quotes Histogram

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


## search_quotes_forex_histogram

> models::SearchQuotesPortfolioHistogram200Response search_quotes_forex_histogram(v1_screener_interval_request)
Search Quotes Histogram

This endpoint allow to use search with complexe queries (keywords, filters, sort etc..) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_interval_request** | [**V1ScreenerIntervalRequest**](V1ScreenerIntervalRequest.md) | Body of the request to search quotes for Forex | [required] |

### Return type

[**models::SearchQuotesPortfolioHistogram200Response**](SearchQuotesPortfolioHistogram_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

