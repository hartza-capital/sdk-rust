# \InstrumentsApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**instrument**](InstrumentsApi.md#instrument) | **GET** /v1/instrument/{ticker} | Get Instrument properties
[**options_instrument**](InstrumentsApi.md#options_instrument) | **OPTIONS** /v1/instrument/{ticker} | Options Instrument



## instrument

> models::Instrument200Response instrument(ticker)
Get Instrument properties

This endpoint returns the properties of the instrument: - General Properties (Ticker, Referencies (ISIN, CIK), type of asset...), - Exchange Properties (Exchange, Currency and status of exchange), - Activities (Look TRBC Classification, https://en.wikipedia.org/wiki/The_Refinitiv_Business_Classification) - Last Quote (EOD, End of Day), - Last 5 years of Dividends, - Statistics (Yield, Beta, Volumes Avg...), - Contact (Email, Physical Address...), 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | Ticker of the instrument | [required] |

### Return type

[**models::Instrument200Response**](Instrument_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_instrument

> options_instrument(ticker)
Options Instrument

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | Ticker of the instrument | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

