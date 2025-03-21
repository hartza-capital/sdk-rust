# \InstrumentsSplitsApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**last_split**](InstrumentsSplitsApi.md#last_split) | **GET** /v1/split/{id}/last | Last Split for Instrument
[**options_last_split**](InstrumentsSplitsApi.md#options_last_split) | **OPTIONS** /v1/split/{id}/last | Options Last Split for Instrument
[**options_splits**](InstrumentsSplitsApi.md#options_splits) | **OPTIONS** /v1/splits | Options Search Splits for Instrument
[**search_splits**](InstrumentsSplitsApi.md#search_splits) | **POST** /v1/splits | Search Instruments Splits by interval and arguments



## last_split

> models::V1SplitResponse last_split(id)
Last Split for Instrument

Permits to get the last split received by the shareholder for the specific instrument.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the instrument to retrieve | [required] |

### Return type

[**models::V1SplitResponse**](v1SplitResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_last_split

> options_last_split(id)
Options Last Split for Instrument

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


## options_splits

> options_splits()
Options Search Splits for Instrument

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


## search_splits

> models::SearchSplits200Response search_splits(v1_screener_np_request)
Search Instruments Splits by interval and arguments

Permits to search splits by interval and arguments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_np_request** | [**V1ScreenerNpRequest**](V1ScreenerNpRequest.md) | Body of the request to search splits by interval and arguments | [required] |

### Return type

[**models::SearchSplits200Response**](SearchSplits_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

