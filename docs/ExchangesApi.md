# \ExchangesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**exchange_by_ticker**](ExchangesApi.md#exchange_by_ticker) | **GET** /v1/exchange/{ticker} | Get Exchange properties
[**exchanges_status**](ExchangesApi.md#exchanges_status) | **GET** /v1/exchanges/status | List Exchanges Status
[**options_exchange_by_ticker**](ExchangesApi.md#options_exchange_by_ticker) | **OPTIONS** /v1/exchange/{ticker} | Options Exchange properties
[**options_exchanges**](ExchangesApi.md#options_exchanges) | **OPTIONS** /v1/exchanges | Options Search Exchanges
[**options_list_exchanges_status**](ExchangesApi.md#options_list_exchanges_status) | **OPTIONS** /v1/exchanges/status | Options Exchanges Status
[**search_exchanges**](ExchangesApi.md#search_exchanges) | **POST** /v1/exchanges | Search Exchanges with queries or filters



## exchange_by_ticker

> models::ExchangeByTicker200Response exchange_by_ticker(ticker)
Get Exchange properties

This endpoint describe: - General informations (Ticker Alternative, primary Index etc...), - Indexes availables, - Statistics (MarketCap global, volume and instrument counter), - TimeZone, - SizeCap (division of business trails), - Holidays, - Trading Hours and Status. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of exchange | [required] |

### Return type

[**models::ExchangeByTicker200Response**](ExchangeByTicker_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exchanges_status

> models::ExchangesStatus200Response exchanges_status()
List Exchanges Status

This endpoint return the list of Exchanges with their status. 

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


## options_exchange_by_ticker

> options_exchange_by_ticker(ticker)
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


## options_exchanges

> options_exchanges()
Options Search Exchanges

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


## options_list_exchanges_status

> options_list_exchanges_status()
Options Exchanges Status

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


## search_exchanges

> models::SearchExchanges200Response search_exchanges(search_instruments_request)
Search Exchanges with queries or filters

This endpoint allow to use search with complexe queries (keywords, filters, sort etc..) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_instruments_request** | Option<[**SearchInstrumentsRequest**](SearchInstrumentsRequest.md)> | Some Description |  |

### Return type

[**models::SearchExchanges200Response**](SearchExchanges_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

