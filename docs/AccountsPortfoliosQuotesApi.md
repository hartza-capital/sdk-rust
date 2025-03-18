# \AccountsPortfoliosQuotesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_quotes_portfolio_histogram**](AccountsPortfoliosQuotesApi.md#search_quotes_portfolio_histogram) | **POST** /v1/portfolios/histogram | Search Quotes Histogram by Portfolio and period
[**search_quotes_portfolios**](AccountsPortfoliosQuotesApi.md#search_quotes_portfolios) | **POST** /v1/portfolios | Search Quotes by Portfolio and period



## search_quotes_portfolio_histogram

> models::SearchQuotesPortfolioHistogram200Response search_quotes_portfolio_histogram(v1_screener_interval_request)
Search Quotes Histogram by Portfolio and period

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_interval_request** | [**V1ScreenerIntervalRequest**](V1ScreenerIntervalRequest.md) | Body of the request to search quotes histogram by Portfolio and period | [required] |

### Return type

[**models::SearchQuotesPortfolioHistogram200Response**](SearchQuotesPortfolioHistogram_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_quotes_portfolios

> models::SearchQuotesPortfolios200Response search_quotes_portfolios(v1_screener_interval_request)
Search Quotes by Portfolio and period

Search Quotes permit to search quotes by Portfolio and period

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_interval_request** | [**V1ScreenerIntervalRequest**](V1ScreenerIntervalRequest.md) | Body of the request to search quotes by Portfolio and period | [required] |

### Return type

[**models::SearchQuotesPortfolios200Response**](SearchQuotesPortfolios_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

