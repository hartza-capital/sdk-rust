# \InstrumentsBacktestApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**backtest_instruments**](InstrumentsBacktestApi.md#backtest_instruments) | **POST** /v1/backtest/instruments | Backtest Strategy for instruments by interval and arguments
[**last_backtest_instrument**](InstrumentsBacktestApi.md#last_backtest_instrument) | **GET** /v1/backtest/{strategy}/instrument/{ticker}/last | Get last trend available
[**lasts_backtest_instruments**](InstrumentsBacktestApi.md#lasts_backtest_instruments) | **POST** /v1/backtest/instruments/lasts | List lasts backtest available for instruments and strategies
[**options_backtest_instruments**](InstrumentsBacktestApi.md#options_backtest_instruments) | **OPTIONS** /v1/backtest/instruments | Options Backtest Strategy for instruments
[**options_last_backtest_instrument**](InstrumentsBacktestApi.md#options_last_backtest_instrument) | **OPTIONS** /v1/backtest/{strategy}/instrument/{ticker}/last | Options last trend available
[**options_lasts_backtest_instruments**](InstrumentsBacktestApi.md#options_lasts_backtest_instruments) | **OPTIONS** /v1/backtest/instruments/lasts | Options lasts backtest available



## backtest_instruments

> models::BacktestInstruments200Response backtest_instruments(x_account, backtest_instruments_request, accumulation)
Backtest Strategy for instruments by interval and arguments

This endpoint return the backtest received by the shareholder for instruments and strategies. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account** | **String** | Account ID | [required] |
**backtest_instruments_request** | [**BacktestInstrumentsRequest**](BacktestInstrumentsRequest.md) | SimulatorStrategiesBacktestRequest is used to specify the request for the SimulatorStrategiesBacktest API. | [required] |
**accumulation** | Option<**bool**> | Accumulation activation |  |[default to false]

### Return type

[**models::BacktestInstruments200Response**](BacktestInstruments_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## last_backtest_instrument

> models::V1BacktestStrategiesResult last_backtest_instrument(ticker, strategy)
Get last trend available

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


## lasts_backtest_instruments

> models::LastsBacktestInstruments200Response lasts_backtest_instruments(lasts_quotes_instruments_strategy_request)
List lasts backtest available for instruments and strategies

This endpoint return the lasts backtest received by the shareholder for instruments and strategies. 

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


## options_backtest_instruments

> options_backtest_instruments()
Options Backtest Strategy for instruments

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


## options_last_backtest_instrument

> options_last_backtest_instrument(ticker, strategy)
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


## options_lasts_backtest_instruments

> options_lasts_backtest_instruments()
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

