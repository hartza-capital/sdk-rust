# \IntradayQuotesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**option_quotes_intraday_instruments**](IntradayQuotesApi.md#option_quotes_intraday_instruments) | **OPTIONS** /v1/intraday/instruments | Options Search Intraday Quotes by Instruments and period
[**search_quotes_intraday_instruments**](IntradayQuotesApi.md#search_quotes_intraday_instruments) | **POST** /v1/intraday/instruments | Search Intraday Quotes by Instruments and period



## option_quotes_intraday_instruments

> option_quotes_intraday_instruments()
Options Search Intraday Quotes by Instruments and period

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


## search_quotes_intraday_instruments

> models::SearchQuotesPortfolios200Response search_quotes_intraday_instruments(search_quotes_intraday_instruments_request)
Search Intraday Quotes by Instruments and period

This endpoint return a list of Intraday Quotes aggregated by interval (5 minutes, 15 minutes, 30 minutes, 1 hour, 4 hours, 8 hours and 1 day). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_quotes_intraday_instruments_request** | [**SearchQuotesIntradayInstrumentsRequest**](SearchQuotesIntradayInstrumentsRequest.md) | IntradayQuotesRequest is used to specify the request for the IntradayQuotes API. | [required] |

### Return type

[**models::SearchQuotesPortfolios200Response**](SearchQuotesPortfolios_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

