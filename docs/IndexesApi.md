# \IndexesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**index_by_ticker**](IndexesApi.md#index_by_ticker) | **GET** /v1/index/{ticker} | Get Index by Ticker
[**options_index_by_ticker**](IndexesApi.md#options_index_by_ticker) | **OPTIONS** /v1/index/{ticker} | Options Get Index
[**options_search_indexes**](IndexesApi.md#options_search_indexes) | **OPTIONS** /v1/indexes | Options Search Indexes
[**search_indexes**](IndexesApi.md#search_indexes) | **POST** /v1/indexes | Search Indexes



## index_by_ticker

> models::IndexByTicker200Response index_by_ticker(ticker)
Get Index by Ticker

This endpoint permit to receive: - General informations (Name, Exchange...), - Activities, - SizeCap (majority), - Last Quote of the day, - Statistics. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of index | [required] |

### Return type

[**models::IndexByTicker200Response**](IndexByTicker_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_index_by_ticker

> options_index_by_ticker(ticker)
Options Get Index

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of index | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_search_indexes

> options_search_indexes()
Options Search Indexes

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


## search_indexes

> models::SearchIndexes200Response search_indexes(search_instruments_request)
Search Indexes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_instruments_request** | Option<[**SearchInstrumentsRequest**](SearchInstrumentsRequest.md)> | Some Description |  |

### Return type

[**models::SearchIndexes200Response**](SearchIndexes_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

