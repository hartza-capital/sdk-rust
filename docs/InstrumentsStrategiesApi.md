# \InstrumentsStrategiesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**last_quote_strategy_instrument_by_ticker**](InstrumentsStrategiesApi.md#last_quote_strategy_instrument_by_ticker) | **GET** /v1/eod/instrument/{ticker}/strategy/{strategy}/last | Get Last Strategy Quote for Instrument
[**lasts_quotes_instruments_strategy**](InstrumentsStrategiesApi.md#lasts_quotes_instruments_strategy) | **POST** /v1/eod/instruments/strategy/lasts | Get Lasts Strategy Quotes for Instruments
[**options_last_quote_strategy_instrument_by_ticker**](InstrumentsStrategiesApi.md#options_last_quote_strategy_instrument_by_ticker) | **OPTIONS** /v1/eod/instrument/{ticker}/strategy/{strategy}/last | Options Last Strategy Quote for Instrument
[**options_lasts_quotes_instruments_strategy**](InstrumentsStrategiesApi.md#options_lasts_quotes_instruments_strategy) | **OPTIONS** /v1/eod/instruments/strategy/lasts | Options Lasts Strategy Quotes for Instruments
[**options_search_quotes_instruments_strategy**](InstrumentsStrategiesApi.md#options_search_quotes_instruments_strategy) | **OPTIONS** /v1/eod/instruments/strategies | Options Strategy Quotes for Instruments
[**search_quotes_instruments_strategy**](InstrumentsStrategiesApi.md#search_quotes_instruments_strategy) | **POST** /v1/eod/instruments/strategies | Search Strategy Quotes for Instruments



## last_quote_strategy_instrument_by_ticker

> models::V1StrategiesQuoteResponse last_quote_strategy_instrument_by_ticker(ticker, strategy)
Get Last Strategy Quote for Instrument

This endpoint return the last quote received by the shareholder for the specific instrument and strategy. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | id of instrument | [required] |
**strategy** | **String** | ticker name of strategy | [required] |

### Return type

[**models::V1StrategiesQuoteResponse**](v1StrategiesQuoteResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lasts_quotes_instruments_strategy

> models::LastsQuotesInstrumentsStrategy200Response lasts_quotes_instruments_strategy(lasts_quotes_instruments_strategy_request)
Get Lasts Strategy Quotes for Instruments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lasts_quotes_instruments_strategy_request** | [**LastsQuotesInstrumentsStrategyRequest**](LastsQuotesInstrumentsStrategyRequest.md) | Some Description | [required] |

### Return type

[**models::LastsQuotesInstrumentsStrategy200Response**](LastsQuotesInstrumentsStrategy_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_last_quote_strategy_instrument_by_ticker

> options_last_quote_strategy_instrument_by_ticker(ticker, strategy)
Options Last Strategy Quote for Instrument

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


## options_lasts_quotes_instruments_strategy

> options_lasts_quotes_instruments_strategy()
Options Lasts Strategy Quotes for Instruments

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


## options_search_quotes_instruments_strategy

> options_search_quotes_instruments_strategy()
Options Strategy Quotes for Instruments

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


## search_quotes_instruments_strategy

> models::SearchQuotesInstrumentsStrategy200Response search_quotes_instruments_strategy(v1_screener_np_request)
Search Strategy Quotes for Instruments

This endpoint return a list of Quotes aggregated by interval (daily, weekly, monthly, quartely, yearly). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_np_request** | [**V1ScreenerNpRequest**](V1ScreenerNpRequest.md) | Body of the request to search quotes by instruments and period | [required] |

### Return type

[**models::SearchQuotesInstrumentsStrategy200Response**](SearchQuotesInstrumentsStrategy_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

