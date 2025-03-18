# \InstrumentsQuotesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**last_quote_instrument_by_ticker**](InstrumentsQuotesApi.md#last_quote_instrument_by_ticker) | **GET** /v1/eod/instrument/{ticker}/last | Last Quote for Instrument by Ticker
[**lasts_quotes_instruments**](InstrumentsQuotesApi.md#lasts_quotes_instruments) | **GET** /v1/eod/instruments/lasts | List lasts quote for Instruments
[**options_last_quote_instrument_by_ticker**](InstrumentsQuotesApi.md#options_last_quote_instrument_by_ticker) | **OPTIONS** /v1/eod/instrument/{ticker}/last | Options Last Quote for Instrument
[**options_lasts_quotes_instruments**](InstrumentsQuotesApi.md#options_lasts_quotes_instruments) | **OPTIONS** /v1/eod/instruments/lasts | Options List lasts quote for Instruments
[**options_quotes_instruments**](InstrumentsQuotesApi.md#options_quotes_instruments) | **OPTIONS** /v1/eod/instruments | Options Search Quotes by Instruments and period
[**search_quotes_instruments**](InstrumentsQuotesApi.md#search_quotes_instruments) | **POST** /v1/eod/instruments | Search Quotes by Instruments and period



## last_quote_instrument_by_ticker

> models::V1QuoteResponse last_quote_instrument_by_ticker(ticker)
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


## lasts_quotes_instruments

> models::LastsQuotesInstruments200Response lasts_quotes_instruments(tickers)
List lasts quote for Instruments

This endpoint return the lasts quotes received by the shareholder for instruments. 

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


## options_last_quote_instrument_by_ticker

> options_last_quote_instrument_by_ticker(ticker)
Options Last Quote for Instrument

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


## options_lasts_quotes_instruments

> options_lasts_quotes_instruments()
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


## options_quotes_instruments

> options_quotes_instruments()
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


## search_quotes_instruments

> models::SearchQuotesPortfolios200Response search_quotes_instruments(v1_screener_interval_request)
Search Quotes by Instruments and period

This endpoint return a list of Quotes aggregated by interval (daily, weekly, monthly, quartely, yearly). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_interval_request** | [**V1ScreenerIntervalRequest**](V1ScreenerIntervalRequest.md) | Body of the request to search quotes by instruments and period | [required] |

### Return type

[**models::SearchQuotesPortfolios200Response**](SearchQuotesPortfolios_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

