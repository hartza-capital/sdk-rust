# \PortfoliosApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**options_portfolios**](PortfoliosApi.md#options_portfolios) | **OPTIONS** /v1/portfolios | Options Portfolios methods
[**portfolios**](PortfoliosApi.md#portfolios) | **GET** /v1/portfolios | List Portfolios



## options_portfolios

> options_portfolios()
Options Portfolios methods

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


## portfolios

> models::Portfolios200Response portfolios(x_account)
List Portfolios

Permits to list positions, cash and value of the portfolios

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account** | **String** | Account ID | [required] |

### Return type

[**models::Portfolios200Response**](Portfolios_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

