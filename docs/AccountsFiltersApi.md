# \AccountsFiltersApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_filter**](AccountsFiltersApi.md#create_filter) | **POST** /v1/filters | Create Account Filter
[**delete_filter_by_ticker**](AccountsFiltersApi.md#delete_filter_by_ticker) | **DELETE** /v1/filter/{ticker} | Delete Account Filter properties
[**filter_by_ticker**](AccountsFiltersApi.md#filter_by_ticker) | **GET** /v1/filter/{ticker} | Get Account Filter properties
[**filters**](AccountsFiltersApi.md#filters) | **GET** /v1/filters | List Account Filters properties
[**options_filter_by_ticker**](AccountsFiltersApi.md#options_filter_by_ticker) | **OPTIONS** /v1/filter/{ticker} | Options Account Filter
[**options_filters**](AccountsFiltersApi.md#options_filters) | **OPTIONS** /v1/filters | Options Filters
[**update_filter_by_ticker**](AccountsFiltersApi.md#update_filter_by_ticker) | **PUT** /v1/filter/{ticker} | Update Account Filter properties



## create_filter

> models::V1FilterResponse create_filter(x_account, create_filter_request)
Create Account Filter

Create Filter permit to create a new filter with properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account** | **String** | Account ID | [required] |
**create_filter_request** | [**CreateFilterRequest**](CreateFilterRequest.md) | CreateFiltersRequest is used to specify the request for the CreateFilters API. | [required] |

### Return type

[**models::V1FilterResponse**](v1FilterResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_filter_by_ticker

> delete_filter_by_ticker(x_account, ticker)
Delete Account Filter properties

Delete Account Filter permit to delete properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account** | **String** | Account ID | [required] |
**ticker** | **String** | The Ticker of the filter to retrieve | [required] |

### Return type

 (empty response body)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## filter_by_ticker

> models::V1FilterResponse filter_by_ticker(x_account, ticker)
Get Account Filter properties

Get Account Filter permit to receive properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account** | **String** | Account ID | [required] |
**ticker** | **String** | The Ticker of the filter to retrieve | [required] |

### Return type

[**models::V1FilterResponse**](v1FilterResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## filters

> models::Filters200Response filters(x_account, items, page)
List Account Filters properties

List Filters permit to list all filters with pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account** | **String** | Account ID | [required] |
**items** | **i64** | number of items returns | [required] |[default to 50]
**page** | **i64** | page number | [required] |[default to 1]

### Return type

[**models::Filters200Response**](Filters_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_filter_by_ticker

> options_filter_by_ticker(ticker)
Options Account Filter

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | The Ticker of the filter to retrieve | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_filters

> options_filters()
Options Filters

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


## update_filter_by_ticker

> models::V1FilterResponse update_filter_by_ticker(x_account, ticker, create_filter_request)
Update Account Filter properties

Update Filter permit to update properties of filter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account** | **String** | Account ID | [required] |
**ticker** | **String** | The Ticker of the filter to retrieve | [required] |
**create_filter_request** | [**CreateFilterRequest**](CreateFilterRequest.md) | CreateFiltersRequest is used to specify the request for the CreateFilters API. | [required] |

### Return type

[**models::V1FilterResponse**](v1FilterResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

