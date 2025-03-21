# \BacktestApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**backtest**](BacktestApi.md#backtest) | **POST** /v1/backtest | Backtest Strategy
[**last_backtest**](BacktestApi.md#last_backtest) | **GET** /v1/backtest/{strategy}/{ticker}/last | Last trend available
[**lasts_backtest**](BacktestApi.md#lasts_backtest) | **POST** /v1/backtest/lasts | Lasts backtest available
[**options_backtest**](BacktestApi.md#options_backtest) | **OPTIONS** /v1/backtest | Options Backtest Strategy
[**options_last_backtest**](BacktestApi.md#options_last_backtest) | **OPTIONS** /v1/backtest/{strategy}/{ticker}/last | Options last trend available
[**options_lasts_backtest**](BacktestApi.md#options_lasts_backtest) | **OPTIONS** /v1/backtest/lasts | Options lasts backtest available



## backtest

> models::Backtest200Response backtest(x_account, backtest_request, accumulation)
Backtest Strategy

Permits to backtest a strategy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account** | **String** | Account ID | [required] |
**backtest_request** | [**BacktestRequest**](BacktestRequest.md) | SimulatorStrategiesBacktestRequest is used to specify the request for the SimulatorStrategiesBacktest API. | [required] |
**accumulation** | Option<**bool**> | Accumulation activation |  |[default to false]

### Return type

[**models::Backtest200Response**](Backtest_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## last_backtest

> models::V1BacktestStrategiesResult last_backtest(ticker, strategy)
Last trend available

Permits to get the last trend received by the shareholder for instruments and strategies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | id of instrument | [required] |
**strategy** | **String** | ticker name of strategy | [required] |

### Return type

[**models::V1BacktestStrategiesResult**](v1BacktestStrategiesResult.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lasts_backtest

> models::LastsBacktest200Response lasts_backtest(lasts_strategy_quotes_request)
Lasts backtest available

Permits to get the last trends received by the shareholder for instruments and strategies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lasts_strategy_quotes_request** | [**LastsStrategyQuotesRequest**](LastsStrategyQuotesRequest.md) | Some Description | [required] |

### Return type

[**models::LastsBacktest200Response**](LastsBacktest_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_backtest

> options_backtest()
Options Backtest Strategy

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


## options_last_backtest

> options_last_backtest(ticker, strategy)
Options last trend available

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | id of instrument | [required] |
**strategy** | **String** | ticker name of strategy | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_lasts_backtest

> options_lasts_backtest()
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

