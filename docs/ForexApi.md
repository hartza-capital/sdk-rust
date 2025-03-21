# \ForexApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**forex**](ForexApi.md#forex) | **GET** /v1/forex/{ticker} | Get Forex properties
[**options_forex**](ForexApi.md#options_forex) | **OPTIONS** /v1/forex/{ticker} | Options Forex properties



## forex

> models::Forex200Response forex(ticker)
Get Forex properties

Permits to get the forex properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of currency exchanged | [required] |

### Return type

[**models::Forex200Response**](Forex_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_forex

> options_forex(ticker)
Options Forex properties

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

