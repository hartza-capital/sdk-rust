# \ForexStrategiesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**last_quote_forex_by_ticker_and_strategy**](ForexStrategiesApi.md#last_quote_forex_by_ticker_and_strategy) | **GET** /v1/eod/forex/{ticker}/strategy/{strategy}/last | Get Last Strategy Quote for Forex
[**lasts_quotes_forex_by_tickers_and_strategy**](ForexStrategiesApi.md#lasts_quotes_forex_by_tickers_and_strategy) | **POST** /v1/eod/forex/strategy/lasts | Get Lasts Strategy Quotes for Forex
[**options_get_lasts_quotes_forex_by_tickers_and_strategy**](ForexStrategiesApi.md#options_get_lasts_quotes_forex_by_tickers_and_strategy) | **OPTIONS** /v1/eod/forex/strategy/lasts | Options Lasts Strategy Quotes for Forex
[**options_last_quote_forex_by_ticker_and_strategy**](ForexStrategiesApi.md#options_last_quote_forex_by_ticker_and_strategy) | **OPTIONS** /v1/eod/forex/{ticker}/strategy/{strategy}/last | Options Last Strategy Quote for Forex
[**options_search_quotes_forexby_ticker_and_strategy**](ForexStrategiesApi.md#options_search_quotes_forexby_ticker_and_strategy) | **OPTIONS** /v1/eod/forex/strategies | Options Strategy Quotes for Forex
[**search_quotes_forex_by_ticker_and_strategy**](ForexStrategiesApi.md#search_quotes_forex_by_ticker_and_strategy) | **POST** /v1/eod/forex/strategies | Search Quotes for Forex by Ticker and Strategy



## last_quote_forex_by_ticker_and_strategy

> models::V1StrategiesQuoteResponse last_quote_forex_by_ticker_and_strategy(ticker, strategy)
Get Last Strategy Quote for Forex

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of currency exchanged | [required] |
**strategy** | **String** | ticker name of strategy | [required] |

### Return type

[**models::V1StrategiesQuoteResponse**](v1StrategiesQuoteResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lasts_quotes_forex_by_tickers_and_strategy

> models::LastsQuotesInstrumentsStrategy200Response lasts_quotes_forex_by_tickers_and_strategy(lasts_quotes_instruments_strategy_request)
Get Lasts Strategy Quotes for Forex

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


## options_get_lasts_quotes_forex_by_tickers_and_strategy

> options_get_lasts_quotes_forex_by_tickers_and_strategy()
Options Lasts Strategy Quotes for Forex

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


## options_last_quote_forex_by_ticker_and_strategy

> options_last_quote_forex_by_ticker_and_strategy(ticker, strategy)
Options Last Strategy Quote for Forex

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


## options_search_quotes_forexby_ticker_and_strategy

> options_search_quotes_forexby_ticker_and_strategy()
Options Strategy Quotes for Forex

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


## search_quotes_forex_by_ticker_and_strategy

> models::SearchQuotesInstrumentsStrategy200Response search_quotes_forex_by_ticker_and_strategy(v1_screener_np_request)
Search Quotes for Forex by Ticker and Strategy

This endpoint allow to use search with complexe queries (keywords, filters, sort etc..) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_np_request** | [**V1ScreenerNpRequest**](V1ScreenerNpRequest.md) | Body of the request to search quotes for Forex by Ticker and Strategy | [required] |

### Return type

[**models::SearchQuotesInstrumentsStrategy200Response**](SearchQuotesInstrumentsStrategy_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

