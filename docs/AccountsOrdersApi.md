# \AccountsOrdersApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_orders**](AccountsOrdersApi.md#create_orders) | **POST** /v1/orders | Create Orders by Account
[**options_orders**](AccountsOrdersApi.md#options_orders) | **OPTIONS** /v1/orders | Options Orders
[**orders**](AccountsOrdersApi.md#orders) | **GET** /v1/orders | List Orders by Account



## create_orders

> models::Orders200Response create_orders(x_account, create_orders_request)
Create Orders by Account

Permits to create Orders on the portfolios

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account** | **String** | Account ID | [required] |
**create_orders_request** | [**CreateOrdersRequest**](CreateOrdersRequest.md) | v1OrdersCreateRequest is used to specify the request for the CreateOrders API. | [required] |

### Return type

[**models::Orders200Response**](Orders_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_orders

> options_orders()
Options Orders

Options method is used to describe the communication options for the targeted resource.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orders

> models::Orders200Response orders(x_account)
List Orders by Account

Permits to list Orders status on the portfolios

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account** | **String** | Account ID | [required] |

### Return type

[**models::Orders200Response**](Orders_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

