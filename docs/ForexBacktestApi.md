# \ForexBacktestApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**backtest_forex**](ForexBacktestApi.md#backtest_forex) | **POST** /v1/backtest/forex | Search Backtest Strategy for forex by Ticker and Strategy
[**get_last_backtest_forex**](ForexBacktestApi.md#get_last_backtest_forex) | **GET** /v1/backtest/{strategy}/forex/{ticker}/last | Get Last Backtest for Forex by Ticker and Strategy
[**lasts_backtest_forex**](ForexBacktestApi.md#lasts_backtest_forex) | **POST** /v1/backtest/forex/lasts | Lasts backtest available for Forex by Tickers and Strategy
[**options_backtest_forex**](ForexBacktestApi.md#options_backtest_forex) | **OPTIONS** /v1/backtest/forex | Options Backtest Strategy for forex
[**options_last_backtest_forex**](ForexBacktestApi.md#options_last_backtest_forex) | **OPTIONS** /v1/backtest/{strategy}/forex/{ticker}/last | Options last trend available
[**options_lasts_backtest_forex**](ForexBacktestApi.md#options_lasts_backtest_forex) | **OPTIONS** /v1/backtest/forex/lasts | Options lasts backtest available



## backtest_forex

> models::BacktestInstruments200Response backtest_forex(backtest_instruments_request)
Search Backtest Strategy for forex by Ticker and Strategy

This endpoint allow to use search with complexe queries (keywords, filters, sort etc..) 

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


## get_last_backtest_forex

> models::V1BacktestStrategiesResult get_last_backtest_forex(ticker, strategy)
Get Last Backtest for Forex by Ticker and Strategy

This endpoint return the last backtest received by the shareholder for the targeted Forex and Strategy. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of currency exchanged | [required] |
**strategy** | **String** | ticker name of strategy | [required] |

### Return type

[**models::V1BacktestStrategiesResult**](v1BacktestStrategiesResult.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lasts_backtest_forex

> models::LastsBacktestInstruments200Response lasts_backtest_forex(lasts_quotes_instruments_strategy_request)
Lasts backtest available for Forex by Tickers and Strategy

This endpoint return the lasts backtest received by the shareholder for the targeted Forex and Strategy. 

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


## options_backtest_forex

> options_backtest_forex()
Options Backtest Strategy for forex

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


## options_last_backtest_forex

> options_last_backtest_forex(ticker, strategy)
Options last trend available

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of currency exchanged | [required] |
**strategy** | **String** | ticker name of strategy | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_lasts_backtest_forex

> options_lasts_backtest_forex()
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

