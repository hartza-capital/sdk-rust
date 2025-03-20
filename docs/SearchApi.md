# \SearchApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**options_search**](SearchApi.md#options_search) | **OPTIONS** /v1/search | Options Search
[**search**](SearchApi.md#search) | **POST** /v1/search | Search with queries or filters



## options_search

> options_search()
Options Search

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


## search

> models::Search200Response search(v1_screener_query_request)
Search with queries or filters

This endpoint return a list of Instruments with properties: - General Properties (Ticker, Referencies (ISIN, CIK), type of asset...), - Quote (1 years monthly), - Dividends (Last 5 years). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_query_request** | Option<[**V1ScreenerQueryRequest**](V1ScreenerQueryRequest.md)> | Some Description |  |

### Return type

[**models::Search200Response**](Search_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

