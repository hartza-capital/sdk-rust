# \AccountsWatchlistsReportsApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**options_watchlist_report**](AccountsWatchlistsReportsApi.md#options_watchlist_report) | **OPTIONS** /v1/watchlist/report/{proxy} | Options Watchlist Report
[**watchlist_report**](AccountsWatchlistsReportsApi.md#watchlist_report) | **GET** /v1/watchlist/report/{proxy} | Get Watchlist Report properties



## options_watchlist_report

> options_watchlist_report(proxy)
Options Watchlist Report

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


## watchlist_report

> models::WatchlistReport200Response watchlist_report(proxy)
Get Watchlist Report properties

Permits to get properties of Watchlist Report

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proxy** | **uuid::Uuid** | The fields to retrieve on results, separated by comma | [required] |

### Return type

[**models::WatchlistReport200Response**](WatchlistReport_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

