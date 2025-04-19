# \StrategiesQuotesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**last_strategy_quote**](StrategiesQuotesApi.md#last_strategy_quote) | **GET** /v1/quotes/{ticker}/strategy/{strategy}/last | Get Last Strategy Quote
[**lasts_strategy_quotes**](StrategiesQuotesApi.md#lasts_strategy_quotes) | **GET** /v1/quotes/strategy/lasts | Lasts Strategy Quotes for Instruments
[**options_last_strategy_quote**](StrategiesQuotesApi.md#options_last_strategy_quote) | **OPTIONS** /v1/quotes/{ticker}/strategy/{strategy}/last | Options Last Strategy Quote
[**options_lasts_strategy_quotes**](StrategiesQuotesApi.md#options_lasts_strategy_quotes) | **OPTIONS** /v1/quotes/strategy/lasts | Options Lasts Strategy Quotes for Instruments
[**options_search_strategies_quotes**](StrategiesQuotesApi.md#options_search_strategies_quotes) | **OPTIONS** /v1/quotes/strategies | Options Strategy Quotes for Instruments
[**search_strategies_quotes**](StrategiesQuotesApi.md#search_strategies_quotes) | **POST** /v1/quotes/strategies | Search Strategies Quotes for Instruments



## last_strategy_quote

> models::V1StrategiesQuoteResponse last_strategy_quote(ticker, strategy)
Get Last Strategy Quote

Permits to get the last quote received by the shareholder for the specific instrument and strategy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | Internal ticker name of the instrument | [required] |
**strategy** | **String** | ticker name of strategy | [required] |

### Return type

[**models::V1StrategiesQuoteResponse**](v1StrategiesQuoteResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lasts_strategy_quotes

> models::LastsStrategyQuotes200Response lasts_strategy_quotes(strategy, tickers)
Lasts Strategy Quotes for Instruments

Permits to get the last quotes received by the shareholder for the specific instruments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**strategy** | **String** | ticker name of strategy | [required] |
**tickers** | Option<**String**> | Some Description |  |

### Return type

[**models::LastsStrategyQuotes200Response**](LastsStrategyQuotes_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_last_strategy_quote

> options_last_strategy_quote(ticker, strategy)
Options Last Strategy Quote

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | Internal ticker name of the instrument | [required] |
**strategy** | **String** | ticker name of strategy | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_lasts_strategy_quotes

> options_lasts_strategy_quotes()
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


## options_search_strategies_quotes

> options_search_strategies_quotes()
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


## search_strategies_quotes

> models::SearchStrategiesQuotes200Response search_strategies_quotes(v1_screener_np_request)
Search Strategies Quotes for Instruments

Permits to search quotes by instruments and period

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_np_request** | [**V1ScreenerNpRequest**](V1ScreenerNpRequest.md) | Body of the request to search quotes by instruments and period | [required] |

### Return type

[**models::SearchStrategiesQuotes200Response**](SearchStrategiesQuotes_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

