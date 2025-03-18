# \CommoditiesStrategiesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**last_quote_commodity_by_ticker_and_strategy**](CommoditiesStrategiesApi.md#last_quote_commodity_by_ticker_and_strategy) | **GET** /v1/eod/commodity/{ticker}/strategy/{strategy}/last | Last Strategy Quote for commodity
[**lasts_quotes_commodities_strategies**](CommoditiesStrategiesApi.md#lasts_quotes_commodities_strategies) | **POST** /v1/eod/commodities/strategy/lasts | Lasts Strategy Quotes for Commodities
[**options_last_quote_commodity_by_ticker_and_strategy**](CommoditiesStrategiesApi.md#options_last_quote_commodity_by_ticker_and_strategy) | **OPTIONS** /v1/eod/commodity/{ticker}/strategy/{strategy}/last | Options Last Strategy Quote for commodity
[**options_lasts_quotes_commodities_strategies**](CommoditiesStrategiesApi.md#options_lasts_quotes_commodities_strategies) | **OPTIONS** /v1/eod/commodities/strategy/lasts | Options Lasts Strategy Quotes for Commodities
[**options_quotes_commodities_strategies**](CommoditiesStrategiesApi.md#options_quotes_commodities_strategies) | **OPTIONS** /v1/eod/commodities/strategies | Options Strategy Quotes for Commodities
[**search_quotes_commodities_strategies**](CommoditiesStrategiesApi.md#search_quotes_commodities_strategies) | **POST** /v1/eod/commodities/strategies | Search Strategy Quotes for Commodities



## last_quote_commodity_by_ticker_and_strategy

> models::V1StrategiesQuoteResponse last_quote_commodity_by_ticker_and_strategy(ticker, strategy)
Last Strategy Quote for commodity

This endpoint permit to receive the last quote of the day for the targeted commodity and strategy. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of commodity | [required] |
**strategy** | **String** | ticker name of strategy | [required] |

### Return type

[**models::V1StrategiesQuoteResponse**](v1StrategiesQuoteResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lasts_quotes_commodities_strategies

> models::LastsQuotesInstrumentsStrategy200Response lasts_quotes_commodities_strategies(lasts_quotes_instruments_strategy_request)
Lasts Strategy Quotes for Commodities

This endpoint permit to receive the lasts quotes of the day for the targeted commodities and strategies. 

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


## options_last_quote_commodity_by_ticker_and_strategy

> options_last_quote_commodity_by_ticker_and_strategy(ticker, strategy)
Options Last Strategy Quote for commodity

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


## options_lasts_quotes_commodities_strategies

> options_lasts_quotes_commodities_strategies()
Options Lasts Strategy Quotes for Commodities

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


## options_quotes_commodities_strategies

> options_quotes_commodities_strategies()
Options Strategy Quotes for Commodities

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


## search_quotes_commodities_strategies

> models::SearchQuotesInstrumentsStrategy200Response search_quotes_commodities_strategies(v1_screener_np_request)
Search Strategy Quotes for Commodities

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_np_request** | [**V1ScreenerNpRequest**](V1ScreenerNpRequest.md) | Body of the request to search quotes for Commodities | [required] |

### Return type

[**models::SearchQuotesInstrumentsStrategy200Response**](SearchQuotesInstrumentsStrategy_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

