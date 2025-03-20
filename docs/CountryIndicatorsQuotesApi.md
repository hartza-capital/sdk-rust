# \CountryIndicatorsQuotesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**last_quote_country_indicator**](CountryIndicatorsQuotesApi.md#last_quote_country_indicator) | **GET** /v1/country/indicator/{ticker}/last | Last Country Indicators Quote
[**lasts_quotes_country_indicators**](CountryIndicatorsQuotesApi.md#lasts_quotes_country_indicators) | **GET** /v1/country/indicators/lasts | Get Lasts Quotes for Country Indicators
[**options_last_quote_country_indicators**](CountryIndicatorsQuotesApi.md#options_last_quote_country_indicators) | **OPTIONS** /v1/country/indicator/{ticker}/last | Options Last Country Indicators Quote
[**options_lasts_quotes_country_indicators**](CountryIndicatorsQuotesApi.md#options_lasts_quotes_country_indicators) | **OPTIONS** /v1/country/indicators/lasts | Options Lasts Strategy Quotes for Commodities
[**options_search_quotes_country_indicators**](CountryIndicatorsQuotesApi.md#options_search_quotes_country_indicators) | **OPTIONS** /v1/country/indicators | Options Search Quotes for Country Indicators
[**search_quotes_country_indicators**](CountryIndicatorsQuotesApi.md#search_quotes_country_indicators) | **POST** /v1/country/indicators | Search Quotes for Country Indicators



## last_quote_country_indicator

> models::V1CountryIndicatorsQuotesResponseResult last_quote_country_indicator(ticker)
Last Country Indicators Quote

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of country indicator | [required] |

### Return type

[**models::V1CountryIndicatorsQuotesResponseResult**](v1CountryIndicatorsQuotesResponseResult.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lasts_quotes_country_indicators

> models::LastsQuotesCountryIndicators200Response lasts_quotes_country_indicators(tickers)
Get Lasts Quotes for Country Indicators

This endpoint return the lasts quotes received by the shareholder for the targeted Country Indicators. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tickers** | **String** | tickers list of country indicator | [required] |

### Return type

[**models::LastsQuotesCountryIndicators200Response**](LastsQuotesCountryIndicators_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_last_quote_country_indicators

> options_last_quote_country_indicators(ticker)
Options Last Country Indicators Quote

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of country indicator | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_lasts_quotes_country_indicators

> options_lasts_quotes_country_indicators()
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


## options_search_quotes_country_indicators

> options_search_quotes_country_indicators()
Options Search Quotes for Country Indicators

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


## search_quotes_country_indicators

> models::SearchQuotesCountryIndicators200Response search_quotes_country_indicators(v1_screener_np_request)
Search Quotes for Country Indicators

This endpoint permit to receive the quotes of the day for the targeted Country Indicators. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_np_request** | [**V1ScreenerNpRequest**](V1ScreenerNpRequest.md) | Search Quotes for Country Indicators | [required] |

### Return type

[**models::SearchQuotesCountryIndicators200Response**](SearchQuotesCountryIndicators_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

