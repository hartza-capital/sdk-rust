# \AccountsFiltersReportsApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**filter_report_by_id**](AccountsFiltersReportsApi.md#filter_report_by_id) | **GET** /v1/filter/report/{proxy} | Get Account Filter Report properties
[**options_filter_report_by_id**](AccountsFiltersReportsApi.md#options_filter_report_by_id) | **OPTIONS** /v1/filter/report/{proxy} | Options Account Filters Report by Ticker



## filter_report_by_id

> models::FilterReportById200Response filter_report_by_id(proxy)
Get Account Filter Report properties

Get Account Filter Report permit to receive properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proxy** | **uuid::Uuid** | The fields to retrieve on results, separated by comma | [required] |

### Return type

[**models::FilterReportById200Response**](FilterReportByID_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_filter_report_by_id

> options_filter_report_by_id(proxy)
Options Account Filters Report by Ticker

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

