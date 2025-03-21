# \NewsApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**options_search_news**](NewsApi.md#options_search_news) | **OPTIONS** /v1/news | Options Search News
[**search_news**](NewsApi.md#search_news) | **POST** /v1/news | Search News



## options_search_news

> options_search_news()
Options Search News

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


## search_news

> models::SearchNews200Response search_news(v1_screener_query_request)
Search News

Permits to search news by query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_screener_query_request** | Option<[**V1ScreenerQueryRequest**](V1ScreenerQueryRequest.md)> | Some Description |  |

### Return type

[**models::SearchNews200Response**](SearchNews_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

