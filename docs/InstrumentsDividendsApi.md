# \InstrumentsDividendsApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**last_dividend_by_ticker**](InstrumentsDividendsApi.md#last_dividend_by_ticker) | **GET** /v1/dividend/{id}/last | Last Dividend for Instrument
[**options_dividends**](InstrumentsDividendsApi.md#options_dividends) | **OPTIONS** /v1/dividends | Options Search Dividends for Instrument
[**options_last_dividend_by_ticker**](InstrumentsDividendsApi.md#options_last_dividend_by_ticker) | **OPTIONS** /v1/dividend/{id}/last | Options Last Dividend for Instrument
[**search_dividends**](InstrumentsDividendsApi.md#search_dividends) | **POST** /v1/dividends | Search Dividends by interval and arguments



## last_dividend_by_ticker

> models::V1DividendResponse last_dividend_by_ticker(id)
Last Dividend for Instrument

This endpoint returns the last dividend received by the shareholder for the specific instrument. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the instrument to retrieve | [required] |

### Return type

[**models::V1DividendResponse**](v1DividendResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_dividends

> options_dividends()
Options Search Dividends for Instrument

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


## options_last_dividend_by_ticker

> options_last_dividend_by_ticker(id)
Options Last Dividend for Instrument

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the instrument to retrieve | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_dividends

> models::SearchDividends200Response search_dividends(v1_screener_interval_request)
Search Dividends by interval and arguments

This endpoint return: - List Dividends aggregated by interval (monthly, quartely, yearly), - Yield (is a financial ratio that tells you the percentage of a company's share price that it pays out in dividends each year) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_interval_request** | [**V1ScreenerIntervalRequest**](V1ScreenerIntervalRequest.md) | Body of the request to search dividends by interval and arguments | [required] |

### Return type

[**models::SearchDividends200Response**](SearchDividends_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

