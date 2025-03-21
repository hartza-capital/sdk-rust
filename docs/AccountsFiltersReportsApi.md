# \AccountsFiltersReportsApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**filter_report**](AccountsFiltersReportsApi.md#filter_report) | **GET** /v1/filter/report/{proxy} | Get Filter Report properties
[**options_filter_report**](AccountsFiltersReportsApi.md#options_filter_report) | **OPTIONS** /v1/filter/report/{proxy} | Options Filter Report



## filter_report

> models::FilterReport200Response filter_report(proxy)
Get Filter Report properties

Permits to get properties of filter report

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proxy** | **uuid::Uuid** | The fields to retrieve on results, separated by comma | [required] |

### Return type

[**models::FilterReport200Response**](FilterReport_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_filter_report

> options_filter_report(proxy)
Options Filter Report

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proxy** | **uuid::Uuid** | The fields to retrieve on results, separated by comma | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

