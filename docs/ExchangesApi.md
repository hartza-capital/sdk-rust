# \ExchangesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**exchange**](ExchangesApi.md#exchange) | **GET** /v1/exchange/{ticker} | Get Exchange properties by Ticker
[**exchanges_status**](ExchangesApi.md#exchanges_status) | **GET** /v1/exchanges/status | List Exchange Status
[**options_exchange**](ExchangesApi.md#options_exchange) | **OPTIONS** /v1/exchange/{ticker} | Options Exchange properties
[**options_exchanges_status**](ExchangesApi.md#options_exchanges_status) | **OPTIONS** /v1/exchanges/status | Options Exchange Status



## exchange

> models::Exchange200Response exchange(ticker)
Get Exchange properties by Ticker

Permits to get the exchange properties by ticker

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of exchange | [required] |

### Return type

[**models::Exchange200Response**](Exchange_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exchanges_status

> models::ExchangesStatus200Response exchanges_status()
List Exchange Status

Permits to list the exchanges status (open, close, pre-market, post-market...)

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ExchangesStatus200Response**](ExchangesStatus_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_exchange

> options_exchange(ticker)
Options Exchange properties

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of exchange | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_exchanges_status

> options_exchanges_status()
Options Exchange Status

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

