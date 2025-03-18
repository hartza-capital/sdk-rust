# \CommoditiesBacktestApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**backtest_commodities**](CommoditiesBacktestApi.md#backtest_commodities) | **POST** /v1/backtest/commodities | Backtest Strategy for commodities
[**last_backtest_commodity**](CommoditiesBacktestApi.md#last_backtest_commodity) | **GET** /v1/backtest/{strategy}/commodity/{ticker}/last | Get last trend available
[**lasts_backtest_commodities**](CommoditiesBacktestApi.md#lasts_backtest_commodities) | **POST** /v1/backtest/commodities/lasts | Get lasts backtest available
[**options_backtest_commodities**](CommoditiesBacktestApi.md#options_backtest_commodities) | **OPTIONS** /v1/backtest/commodities | Options Backtest Strategy for commodities
[**options_last_backtest_commodity**](CommoditiesBacktestApi.md#options_last_backtest_commodity) | **OPTIONS** /v1/backtest/{strategy}/commodity/{ticker}/last | Options last trend available
[**options_lasts_backtest_commodities**](CommoditiesBacktestApi.md#options_lasts_backtest_commodities) | **OPTIONS** /v1/backtest/commodities/lasts | Options lasts backtest available



## backtest_commodities

> models::BacktestInstruments200Response backtest_commodities(backtest_instruments_request)
Backtest Strategy for commodities

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backtest_instruments_request** | [**BacktestInstrumentsRequest**](BacktestInstrumentsRequest.md) | SimulatorStrategiesBacktestRequest is used to specify the request for the SimulatorStrategiesBacktest API. | [required] |

### Return type

[**models::BacktestInstruments200Response**](BacktestInstruments_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## last_backtest_commodity

> models::V1BacktestStrategiesResult last_backtest_commodity(ticker, strategy)
Get last trend available

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of commodity | [required] |
**strategy** | **String** | ticker name of strategy | [required] |

### Return type

[**models::V1BacktestStrategiesResult**](v1BacktestStrategiesResult.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lasts_backtest_commodities

> models::LastsBacktestInstruments200Response lasts_backtest_commodities(lasts_quotes_instruments_strategy_request)
Get lasts backtest available

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lasts_quotes_instruments_strategy_request** | [**LastsQuotesInstrumentsStrategyRequest**](LastsQuotesInstrumentsStrategyRequest.md) | Some Description | [required] |

### Return type

[**models::LastsBacktestInstruments200Response**](LastsBacktestInstruments_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_backtest_commodities

> options_backtest_commodities()
Options Backtest Strategy for commodities

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


## options_last_backtest_commodity

> options_last_backtest_commodity(ticker, strategy)
Options last trend available

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of commodity | [required] |
**strategy** | **String** | ticker name of strategy | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_lasts_backtest_commodities

> options_lasts_backtest_commodities()
Options lasts backtest available

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

