# \DataQuotesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**last_quote**](DataQuotesApi.md#last_quote) | **GET** /v1/quotes/{ticker}/last | Get Last Quote for Instrument
[**lasts_quotes**](DataQuotesApi.md#lasts_quotes) | **GET** /v1/quotes/lasts | List lasts quotes for data
[**option_intraday_quotes**](DataQuotesApi.md#option_intraday_quotes) | **OPTIONS** /v1/quotes/intraday | Options Search Quotes by Instruments and period
[**options_last_quote**](DataQuotesApi.md#options_last_quote) | **OPTIONS** /v1/quotes/{ticker}/last | Options Last Quote for Instrument by Ticker
[**options_lasts_quotes**](DataQuotesApi.md#options_lasts_quotes) | **OPTIONS** /v1/quotes/lasts | Options List lasts quote for Instruments
[**options_search_quotes**](DataQuotesApi.md#options_search_quotes) | **OPTIONS** /v1/quotes | Options Search Quotes by Instruments and period
[**search_intraday_quotes**](DataQuotesApi.md#search_intraday_quotes) | **POST** /v1/quotes/intraday | Search Intraday Quotes by Instruments and period
[**search_quotes**](DataQuotesApi.md#search_quotes) | **POST** /v1/quotes | Search Quotes in interval and period



## last_quote

> models::V1QuoteResponse last_quote(ticker)
Get Last Quote for Instrument

Permits to get the last quote received by the shareholder for the specific instrument.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | Internal ticker name of the instrument | [required] |

### Return type

[**models::V1QuoteResponse**](v1QuoteResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lasts_quotes

> models::LastsQuotesPortfolios200Response lasts_quotes(tickers)
List lasts quotes for data

Permits to list lasts quotes for the specific data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tickers** | Option<**String**> | Some Description |  |

### Return type

[**models::LastsQuotesPortfolios200Response**](LastsQuotesPortfolios_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## option_intraday_quotes

> option_intraday_quotes()
Options Search Quotes by Instruments and period

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


## options_last_quote

> options_last_quote(ticker)
Options Last Quote for Instrument by Ticker

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | Internal ticker name of the instrument | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_lasts_quotes

> options_lasts_quotes()
Options List lasts quote for Instruments

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


## options_search_quotes

> options_search_quotes()
Options Search Quotes by Instruments and period

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


## search_intraday_quotes

> models::PortfoliosQuotes200Response search_intraday_quotes(search_intraday_quotes_request)
Search Intraday Quotes by Instruments and period

Permits to search intraday quotes by instruments and period (from, to)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_intraday_quotes_request** | [**SearchIntradayQuotesRequest**](SearchIntradayQuotesRequest.md) | IntradayQuotesRequest is used to specify the request for the IntradayQuotes API. | [required] |

### Return type

[**models::PortfoliosQuotes200Response**](PortfoliosQuotes_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_quotes

> models::PortfoliosQuotes200Response search_quotes(v1_screener_interval_request, distribution)
Search Quotes in interval and period

Permits to search quotes by period (from, to)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_interval_request** | [**V1ScreenerIntervalRequest**](V1ScreenerIntervalRequest.md) | Body of the request to search quotes by instruments and period | [required] |
**distribution** | Option<**String**> | Enable distribution |  |[default to false]

### Return type

[**models::PortfoliosQuotes200Response**](PortfoliosQuotes_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

