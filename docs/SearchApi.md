# \SearchApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**options_search**](SearchApi.md#options_search) | **OPTIONS** /v1/search | Options Search
[**search**](SearchApi.md#search) | **POST** /v1/search | Search Instruments, Exchanges, Index, ETFs, Funds, Bonds, Options, Futures, Currencies, Cryptocurrencies



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
Search Instruments, Exchanges, Index, ETFs, Funds, Bonds, Options, Futures, Currencies, Cryptocurrencies

Permits to search instruments, exchanges, index, etfs, funds, bonds, options, futures, currencies, cryptocurrencies by query

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

