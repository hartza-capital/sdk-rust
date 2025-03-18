# \CommoditiesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**commodity_by_ticker**](CommoditiesApi.md#commodity_by_ticker) | **GET** /v1/commodity/{ticker} | Get Commodity properties
[**options_commodities**](CommoditiesApi.md#options_commodities) | **OPTIONS** /v1/commodities | Options Search Commodities
[**options_commodity_by_ticker**](CommoditiesApi.md#options_commodity_by_ticker) | **OPTIONS** /v1/commodity/{ticker} | Options Commodity
[**options_quotes_commodities**](CommoditiesApi.md#options_quotes_commodities) | **OPTIONS** /v1/eod/commodities | Options Search Commodities Quotes
[**search_commodities**](CommoditiesApi.md#search_commodities) | **POST** /v1/commodities | Search Commodities
[**search_quotes_commodities**](CommoditiesApi.md#search_quotes_commodities) | **POST** /v1/eod/commodities | Search Quotes for Commodities



## commodity_by_ticker

> models::CommodityByTicker200Response commodity_by_ticker(ticker)
Get Commodity properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of commodity | [required] |

### Return type

[**models::CommodityByTicker200Response**](CommodityByTicker_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_commodities

> options_commodities()
Options Search Commodities

Options method is used to describe the communication options for the targeted resource.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_commodity_by_ticker

> options_commodity_by_ticker(ticker)
Options Commodity

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of commodity | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_quotes_commodities

> options_quotes_commodities()
Options Search Commodities Quotes

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


## search_commodities

> models::SearchCommodities200Response search_commodities(search_instruments_request)
Search Commodities

This endpoint allow to use search with complexe queries (keywords, filters, sort etc..) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_instruments_request** | Option<[**SearchInstrumentsRequest**](SearchInstrumentsRequest.md)> | Some Description |  |

### Return type

[**models::SearchCommodities200Response**](SearchCommodities_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_quotes_commodities

> models::SearchQuotesPortfolios200Response search_quotes_commodities(v1_screener_interval_request)
Search Quotes for Commodities

This endpoint allow to use search with complexe queries (keywords, filters, sort etc..) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_interval_request** | [**V1ScreenerIntervalRequest**](V1ScreenerIntervalRequest.md) | Body of the request to search quotes for Commodities | [required] |

### Return type

[**models::SearchQuotesPortfolios200Response**](SearchQuotesPortfolios_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

