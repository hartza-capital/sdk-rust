# \ForexApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**forex_by_ticker**](ForexApi.md#forex_by_ticker) | **GET** /v1/forex/{ticker} | Get Forex properties by Ticker
[**options_forex**](ForexApi.md#options_forex) | **OPTIONS** /v1/forex | Options Search Forex
[**options_forex_by_ticker**](ForexApi.md#options_forex_by_ticker) | **OPTIONS** /v1/forex/{ticker} | Options Forex properties
[**search_forex**](ForexApi.md#search_forex) | **POST** /v1/forex | Search Forex with queries or filters



## forex_by_ticker

> models::ForexByTicker200Response forex_by_ticker(ticker)
Get Forex properties by Ticker

This endpoint return: - Global informations (Central Bank, Exchanges/Countries where the currency is used...), - Last Quote (last End of Day (EOD)), - Stats (Pricing). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of currency exchanged | [required] |

### Return type

[**models::ForexByTicker200Response**](ForexByTicker_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_forex

> options_forex()
Options Search Forex

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


## options_forex_by_ticker

> options_forex_by_ticker(ticker)
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


## search_forex

> models::SearchForex200Response search_forex(search_instruments_request)
Search Forex with queries or filters

This endpoint allow to use search with complexe queries (keywords, filters, sort etc..) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_instruments_request** | Option<[**SearchInstrumentsRequest**](SearchInstrumentsRequest.md)> | Some Description |  |

### Return type

[**models::SearchForex200Response**](SearchForex_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

