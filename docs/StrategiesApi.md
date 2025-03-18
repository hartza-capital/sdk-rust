# \StrategiesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**options_strategies**](StrategiesApi.md#options_strategies) | **OPTIONS** /v1/strategies | Options Strategies
[**options_strategy_by_ticker**](StrategiesApi.md#options_strategy_by_ticker) | **OPTIONS** /v1/strategy/{strategy} | Options Strategy properties
[**strategies**](StrategiesApi.md#strategies) | **GET** /v1/strategies | List Strategies properties
[**strategy_by_ticker**](StrategiesApi.md#strategy_by_ticker) | **GET** /v1/strategy/{strategy} | Get Strategy properties



## options_strategies

> options_strategies()
Options Strategies

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


## options_strategy_by_ticker

> options_strategy_by_ticker(strategy)
Options Strategy properties

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**strategy** | **String** | ticker name of strategy | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## strategies

> models::Strategies200Response strategies(items, page)
List Strategies properties

List Strategies permit to list all strategies with pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**items** | **i64** | number of items returns | [required] |[default to 50]
**page** | **i64** | page number | [required] |[default to 1]

### Return type

[**models::Strategies200Response**](Strategies_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## strategy_by_ticker

> models::V1StrategyResponse strategy_by_ticker(strategy)
Get Strategy properties

Get Strategy permit to receive properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**strategy** | **String** | ticker name of strategy | [required] |

### Return type

[**models::V1StrategyResponse**](v1StrategyResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

