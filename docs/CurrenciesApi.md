# \CurrenciesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**currency**](CurrenciesApi.md#currency) | **GET** /v1/currency/{ticker} | Get Currency properties
[**options_currency**](CurrenciesApi.md#options_currency) | **OPTIONS** /v1/currency/{ticker} | Options Currency properties



## currency

> models::V1CurrencyResponse currency(ticker)
Get Currency properties

Permits to get the currency properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of currency | [required] |

### Return type

[**models::V1CurrencyResponse**](v1CurrencyResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_currency

> options_currency(ticker)
Options Currency properties

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

