# \AccountsWatchlistsApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_watchlist**](AccountsWatchlistsApi.md#create_watchlist) | **POST** /v1/watchlists | Create Account Watchlist
[**delete_watchlist**](AccountsWatchlistsApi.md#delete_watchlist) | **DELETE** /v1/watchlist/{ticker} | Delete Watchlist
[**options_watchlist**](AccountsWatchlistsApi.md#options_watchlist) | **OPTIONS** /v1/watchlist/{ticker} | Options Account Watchlist
[**options_watchlists**](AccountsWatchlistsApi.md#options_watchlists) | **OPTIONS** /v1/watchlists | Options Watchlists
[**update_watchlist**](AccountsWatchlistsApi.md#update_watchlist) | **PUT** /v1/watchlist/{ticker} | Update Watchlist properties
[**watchlist**](AccountsWatchlistsApi.md#watchlist) | **GET** /v1/watchlist/{ticker} | Get Watchlist properties
[**watchlists**](AccountsWatchlistsApi.md#watchlists) | **GET** /v1/watchlists | List Watchlists



## create_watchlist

> models::V1WatchlistResponse create_watchlist(x_account, create_watchlist_request)
Create Account Watchlist

Permits to create a new watchlist for the account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account** | **String** | Account ID | [required] |
**create_watchlist_request** | [**CreateWatchlistRequest**](CreateWatchlistRequest.md) | CreateFiltersRequest is used to specify the request for the CreateFilters API. | [required] |

### Return type

[**models::V1WatchlistResponse**](v1WatchlistResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_watchlist

> delete_watchlist(x_account, ticker)
Delete Watchlist

Permits to delete watchlist for the account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account** | **String** | Account ID | [required] |
**ticker** | **String** | The Ticker of the filter to retrieve | [required] |

### Return type

 (empty response body)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_watchlist

> options_watchlist(ticker)
Options Account Watchlist

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | The Ticker of the filter to retrieve | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_watchlists

> options_watchlists()
Options Watchlists

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


## update_watchlist

> models::V1WatchlistResponse update_watchlist(x_account, ticker, update_watchlist_request)
Update Watchlist properties

Permits to update properties of watchlist for the account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account** | **String** | Account ID | [required] |
**ticker** | **String** | The Ticker of the filter to retrieve | [required] |
**update_watchlist_request** | [**UpdateWatchlistRequest**](UpdateWatchlistRequest.md) | CreateFiltersRequest is used to specify the request for the CreateFilters API. | [required] |

### Return type

[**models::V1WatchlistResponse**](v1WatchlistResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## watchlist

> models::V1WatchlistResponse watchlist(x_account, ticker)
Get Watchlist properties

Permits to get properties of watchlist for the account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account** | **String** | Account ID | [required] |
**ticker** | **String** | The Ticker of the filter to retrieve | [required] |

### Return type

[**models::V1WatchlistResponse**](v1WatchlistResponse.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## watchlists

> models::Watchlists200Response watchlists(x_account, items, page)
List Watchlists

Permits to list watchlists for the account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account** | **String** | Account ID | [required] |
**items** | **i64** | number of items returns | [required] |[default to 50]
**page** | **i64** | page number | [required] |[default to 1]

### Return type

[**models::Watchlists200Response**](Watchlists_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

