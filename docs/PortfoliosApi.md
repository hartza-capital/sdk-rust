# \PortfoliosApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**options_portfolio_analysis**](PortfoliosApi.md#options_portfolio_analysis) | **OPTIONS** /v1/portfolios/analysis | Options Portfolios Analysis
[**options_portfolios**](PortfoliosApi.md#options_portfolios) | **OPTIONS** /v1/portfolios | Options Portfolios methods
[**portfolio_analysis**](PortfoliosApi.md#portfolio_analysis) | **GET** /v1/portfolios/analysis | Get Account Portfolio Analysis
[**portfolios**](PortfoliosApi.md#portfolios) | **GET** /v1/portfolios | List Portfolios



## options_portfolio_analysis

> options_portfolio_analysis()
Options Portfolios Analysis

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


## portfolio_analysis

> serde_json::Value portfolio_analysis()
Get Account Portfolio Analysis

Get Account Portfolio Analysis permit to launch the analysis of the portfolio 

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolios

> models::Portfolios200Response portfolios(x_account)
List Portfolios

List Portfolios and their positions cost and value

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

