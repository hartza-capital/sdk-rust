# \PortfoliosAnalysisApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**options_portfolios_analysis**](PortfoliosAnalysisApi.md#options_portfolios_analysis) | **OPTIONS** /v1/portfolios/analysis | Options Portfolios Analysis
[**portfolio_analysis**](PortfoliosAnalysisApi.md#portfolio_analysis) | **GET** /v1/portfolios/analysis | Launch Portfolio Analysis



## options_portfolios_analysis

> options_portfolios_analysis()
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


## portfolio_analysis

> serde_json::Value portfolio_analysis()
Launch Portfolio Analysis

Permits to launch the analysis of the portfolio

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

