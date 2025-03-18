# \InstrumentsApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**instrument_by_id**](InstrumentsApi.md#instrument_by_id) | **GET** /v1/instrument/{id} | Get properties Instrument by ID
[**options_instrument_by_id**](InstrumentsApi.md#options_instrument_by_id) | **OPTIONS** /v1/instrument/{id} | Options Get Instrument by ID
[**options_instruments**](InstrumentsApi.md#options_instruments) | **OPTIONS** /v1/instruments | Options Search Instruments
[**search_instruments**](InstrumentsApi.md#search_instruments) | **POST** /v1/instruments | Search Instruments with queries or filters



## instrument_by_id

> models::InstrumentById200Response instrument_by_id(id)
Get properties Instrument by ID

This endpoint return: - General Properties (Ticker, Referencies (ISIN, CIK), type of asset...), - Exchange Properties (Exchange, Currency and status of exchange), - Activities (Look TRBC Classification, https://en.wikipedia.org/wiki/The_Refinitiv_Business_Classification) - Last Quote (EOD, End of Day), - Last 5 years of Dividends, - Statistics (Yield, Beta, Volumes Avg...), - Contact (Email, Physical Address...), 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the instrument to retrieve | [required] |

### Return type

[**models::InstrumentById200Response**](InstrumentByID_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_instrument_by_id

> options_instrument_by_id(id)
Options Get Instrument by ID

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the instrument to retrieve | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_instruments

> options_instruments()
Options Search Instruments

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


## search_instruments

> models::SearchInstruments200Response search_instruments(search_instruments_request)
Search Instruments with queries or filters

This endpoint return a list of Instruments with properties: - General Properties (Ticker, Referencies (ISIN, CIK), type of asset...), - Quote (1 years monthly), - Dividends (Last 5 years). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_instruments_request** | Option<[**SearchInstrumentsRequest**](SearchInstrumentsRequest.md)> | Some Description |  |

### Return type

[**models::SearchInstruments200Response**](SearchInstruments_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

