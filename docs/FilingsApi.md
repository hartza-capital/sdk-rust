# \FilingsApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**filings_by_cik**](FilingsApi.md#filings_by_cik) | **GET** /v1/filings/us/{proxy} | Get US Filings by CIK (Central Index Key)
[**options_filings_by_cik**](FilingsApi.md#options_filings_by_cik) | **OPTIONS** /v1/filings/us/{proxy} | Options US Filings



## filings_by_cik

> models::FilingsByCik200Response filings_by_cik(proxy)
Get US Filings by CIK (Central Index Key)

This endpoint return the list of US Filings by CIK (Central Index Key). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proxy** | **String** | The fields to retrieve on results, separated by comma | [required] |

### Return type

[**models::FilingsByCik200Response**](FilingsByCIK_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_filings_by_cik

> options_filings_by_cik(proxy)
Options US Filings

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proxy** | **String** | The fields to retrieve on results, separated by comma | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

