# \InstrumentsHistogramApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**options_quotes_histogram_instruments**](InstrumentsHistogramApi.md#options_quotes_histogram_instruments) | **OPTIONS** /v1/eod/instruments/histogram | Options Search Quotes Histogram for Instruments
[**search_quotes_histogram_instruments**](InstrumentsHistogramApi.md#search_quotes_histogram_instruments) | **POST** /v1/eod/instruments/histogram | Search Quotes Histogram by Instruments and period



## options_quotes_histogram_instruments

> options_quotes_histogram_instruments()
Options Search Quotes Histogram for Instruments

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


## search_quotes_histogram_instruments

> models::SearchQuotesPortfolioHistogram200Response search_quotes_histogram_instruments(v1_screener_interval_request)
Search Quotes Histogram by Instruments and period

This endpoint return a list of Quotes Histogram aggregated by interval (daily, weekly, monthly, quartely, yearly). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_interval_request** | [**V1ScreenerIntervalRequest**](V1ScreenerIntervalRequest.md) | Body of the request to search quotes histogram by instruments and period | [required] |

### Return type

[**models::SearchQuotesPortfolioHistogram200Response**](SearchQuotesPortfolioHistogram_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

