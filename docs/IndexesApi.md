# \IndexesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**index**](IndexesApi.md#index) | **GET** /v1/index/{ticker} | Get Index by Ticker
[**options_index**](IndexesApi.md#options_index) | **OPTIONS** /v1/index/{ticker} | Options Get Index



## index

> models::Index200Response index(ticker)
Get Index by Ticker

This endpoint permit to receive: - General informations (Name, Exchange...), - Activities, - SizeCap (majority), - Last Quote of the day, - Statistics. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of index | [required] |

### Return type

[**models::Index200Response**](Index_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_index

> options_index(ticker)
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

