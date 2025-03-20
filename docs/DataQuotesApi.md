# \DataQuotesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**last_quote_by_ticker**](DataQuotesApi.md#last_quote_by_ticker) | **GET** /v1/quotes/{ticker}/last | Last Quote for Instrument by Ticker
[**lasts_quotes**](DataQuotesApi.md#lasts_quotes) | **POST** /v1/quotes/lasts | List lasts quote for Instruments
[**option_intraday_quotes**](DataQuotesApi.md#option_intraday_quotes) | **OPTIONS** /v1/quotes/intraday | Options Search Quotes by Instruments and period
[**options_last_quote_by_ticker**](DataQuotesApi.md#options_last_quote_by_ticker) | **OPTIONS** /v1/quotes/{ticker}/last | Options Last Quote for Instrument by Ticker
[**options_lasts_quotes**](DataQuotesApi.md#options_lasts_quotes) | **OPTIONS** /v1/quotes/lasts | Options List lasts quote for Instruments
[**options_search_quotes**](DataQuotesApi.md#options_search_quotes) | **OPTIONS** /v1/quotes | Options Search Quotes by Instruments and period
[**options_search_quotes_histogram**](DataQuotesApi.md#options_search_quotes_histogram) | **OPTIONS** /v1/quotes/histogram | Options Search Quotes Histogram for Instruments
[**search_intraday_quotes**](DataQuotesApi.md#search_intraday_quotes) | **POST** /v1/quotes/intraday | Search Quotes by Instruments and period
[**search_quotes**](DataQuotesApi.md#search_quotes) | **POST** /v1/quotes | Search Quotes by Instruments and period
[**search_quotes_histogram**](DataQuotesApi.md#search_quotes_histogram) | **POST** /v1/quotes/histogram | Search Quotes Histogram by Instruments and period



## last_quote_by_ticker

> models::V1QuoteResponse last_quote_by_ticker(ticker)
Last Quote for Instrument by Ticker

This endpoint return the last quote received by the shareholder for the specific instrument. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | id of instrument | [required] |

### Return type

[**models::V1QuoteResponse**](v1QuoteResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lasts_quotes

> models::LastsQuotesPortfolios200Response lasts_quotes(lasts_quotes_portfolios_request)
List lasts quote for Instruments

This endpoint return the lasts quotes received by the shareholder for instruments. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lasts_quotes_portfolios_request** | [**LastsQuotesPortfoliosRequest**](LastsQuotesPortfoliosRequest.md) | Some Description | [required] |

### Return type

[**models::LastsQuotesPortfolios200Response**](LastsQuotesPortfolios_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
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


## options_last_quote_by_ticker

> options_last_quote_by_ticker(ticker)
Options Last Quote for Instrument by Ticker

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | id of instrument | [required] |

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


## options_search_quotes_histogram

> options_search_quotes_histogram()
Options Search Quotes Histogram for Instruments

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
Search Quotes by Instruments and period

This endpoint return a list of Quotes aggregated by interval (5 minutes, 15 minutes, 30 minutes, 1 hour, 4 hours, 8 hours and 1 day). 

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

> models::PortfoliosQuotes200Response search_quotes(v1_screener_interval_request)
Search Quotes by Instruments and period

This endpoint return a list of Quotes aggregated by interval (daily, weekly, monthly, quartely, yearly). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_interval_request** | [**V1ScreenerIntervalRequest**](V1ScreenerIntervalRequest.md) | Body of the request to search quotes by instruments and period | [required] |

### Return type

[**models::PortfoliosQuotes200Response**](PortfoliosQuotes_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_quotes_histogram

> models::PortfoliosHistogram200Response search_quotes_histogram(v1_screener_interval_request)
Search Quotes Histogram by Instruments and period

This endpoint return a list of Quotes Histogram aggregated by interval (daily, weekly, monthly, quartely, yearly). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_interval_request** | [**V1ScreenerIntervalRequest**](V1ScreenerIntervalRequest.md) | Body of the request to search quotes histogram by instruments and period | [required] |

### Return type

[**models::PortfoliosHistogram200Response**](PortfoliosHistogram_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

