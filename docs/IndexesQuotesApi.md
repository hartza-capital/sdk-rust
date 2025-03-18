# \IndexesQuotesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**last_quote_index_by_ticker**](IndexesQuotesApi.md#last_quote_index_by_ticker) | **GET** /v1/eod/index/{ticker}/last | Last Quote for Instrument
[**lasts_quotes_index**](IndexesQuotesApi.md#lasts_quotes_index) | **GET** /v1/eod/indexes/lasts | Lasts Indexes Quotes
[**options_index_quote**](IndexesQuotesApi.md#options_index_quote) | **OPTIONS** /v1/eod/index/{ticker}/last | Options Last Quote for Instrument
[**options_lasts_quotes_index**](IndexesQuotesApi.md#options_lasts_quotes_index) | **OPTIONS** /v1/eod/indexes/lasts | Options Lasts Indexes Quotes
[**options_search_quotes_index**](IndexesQuotesApi.md#options_search_quotes_index) | **OPTIONS** /v1/eod/indexes | Options Search Quotes
[**options_search_quotes_indexes_histogram**](IndexesQuotesApi.md#options_search_quotes_indexes_histogram) | **OPTIONS** /v1/eod/indexes/histogram | Options Search Indexes Quotes Histogram
[**search_quotes_index**](IndexesQuotesApi.md#search_quotes_index) | **POST** /v1/eod/indexes | Search Quotes
[**search_quotes_indexes_histogram**](IndexesQuotesApi.md#search_quotes_indexes_histogram) | **POST** /v1/eod/indexes/histogram | Search Indexes Quotes Histogram



## last_quote_index_by_ticker

> models::V1QuoteResponse last_quote_index_by_ticker(ticker)
Last Quote for Instrument

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of index | [required] |

### Return type

[**models::V1QuoteResponse**](v1QuoteResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lasts_quotes_index

> models::LastsQuotesInstruments200Response lasts_quotes_index(tickers)
Lasts Indexes Quotes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tickers** | Option<**String**> | Some Description |  |

### Return type

[**models::LastsQuotesInstruments200Response**](LastsQuotesInstruments_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_index_quote

> options_index_quote(ticker)
Options Last Quote for Instrument

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of index | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_lasts_quotes_index

> options_lasts_quotes_index()
Options Lasts Indexes Quotes

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


## options_search_quotes_index

> options_search_quotes_index()
Options Search Quotes

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


## options_search_quotes_indexes_histogram

> options_search_quotes_indexes_histogram()
Options Search Indexes Quotes Histogram

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


## search_quotes_index

> models::SearchQuotesPortfolios200Response search_quotes_index(v1_screener_interval_request)
Search Quotes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_interval_request** | [**V1ScreenerIntervalRequest**](V1ScreenerIntervalRequest.md) | Body of the request to search quotes for Indexes | [required] |

### Return type

[**models::SearchQuotesPortfolios200Response**](SearchQuotesPortfolios_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_quotes_indexes_histogram

> models::SearchQuotesPortfolioHistogram200Response search_quotes_indexes_histogram(v1_screener_interval_request)
Search Indexes Quotes Histogram

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_interval_request** | [**V1ScreenerIntervalRequest**](V1ScreenerIntervalRequest.md) | Body of the request to search quotes for Indexes | [required] |

### Return type

[**models::SearchQuotesPortfolioHistogram200Response**](SearchQuotesPortfolioHistogram_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

