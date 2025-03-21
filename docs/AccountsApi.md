# \AccountsApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_by_id**](AccountsApi.md#account_by_id) | **GET** /v1/account | Get Account properties by ID
[**accounts**](AccountsApi.md#accounts) | **GET** /v1/accounts | List Accounts properties
[**archive_account_by_id**](AccountsApi.md#archive_account_by_id) | **DELETE** /v1/account | Archive Account by ID
[**create_account**](AccountsApi.md#create_account) | **POST** /v1/accounts | Create Account
[**options_account_by_id**](AccountsApi.md#options_account_by_id) | **OPTIONS** /v1/account | Options Account methods by ID
[**options_accounts**](AccountsApi.md#options_accounts) | **OPTIONS** /v1/accounts | Options List Accounts properties
[**patch_account_by_id**](AccountsApi.md#patch_account_by_id) | **PATCH** /v1/account | Patch Account properties by ID



## account_by_id

> models::AccountById200Response account_by_id(x_account)
Get Account properties by ID

Permits to get Account properties, filters and strategy applicated on the portfolios

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account** | **String** | Account ID | [required] |

### Return type

[**models::AccountById200Response**](AccountByID_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts

> models::Accounts200Response accounts(items, page)
List Accounts properties

Permits to list positions, cash and value of the portfolios

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**items** | **i64** | number of items returns | [required] |[default to 50]
**page** | **i64** | page number | [required] |[default to 1]

### Return type

[**models::Accounts200Response**](Accounts_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## archive_account_by_id

> models::AccountById200Response archive_account_by_id(x_account)
Archive Account by ID

Permits to archive Account and disable strategy applicated on the portfolios

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account** | **String** | Account ID | [required] |

### Return type

[**models::AccountById200Response**](AccountByID_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_account

> models::AccountById200Response create_account(create_account_request)
Create Account

Create Account permit to create a new account with strategy and properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_account_request** | Option<[**CreateAccountRequest**](CreateAccountRequest.md)> | Some Description |  |

### Return type

[**models::AccountById200Response**](AccountByID_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_account_by_id

> options_account_by_id()
Options Account methods by ID

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


## options_accounts

> options_accounts()
Options List Accounts properties

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


## patch_account_by_id

> models::AccountById200Response patch_account_by_id(x_account, patch_account_by_id_request)
Patch Account properties by ID

Permits to patch Account properties, filters and strategy applicated on the portfolios

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account** | **String** | Account ID | [required] |
**patch_account_by_id_request** | Option<[**PatchAccountByIdRequest**](PatchAccountByIdRequest.md)> | Some Description |  |

### Return type

[**models::AccountById200Response**](AccountByID_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

