# \CountriesApi

All URIs are relative to *https://management.api.hartza.capital*

Method | HTTP request | Description
------------- | ------------- | -------------
[**country_by_ticker**](CountriesApi.md#country_by_ticker) | **GET** /v1/country/{ticker} | Get Country properties
[**country_indicators_events_by_ticker**](CountriesApi.md#country_indicators_events_by_ticker) | **GET** /v1/country/{ticker}/events | List Country Events properties
[**options_countries**](CountriesApi.md#options_countries) | **OPTIONS** /v1/countries | Options Search Countries
[**options_country_by_ticker**](CountriesApi.md#options_country_by_ticker) | **OPTIONS** /v1/country/{ticker} | Options Country properties
[**options_country_indicators_events_by_ticker**](CountriesApi.md#options_country_indicators_events_by_ticker) | **OPTIONS** /v1/country/{ticker}/events | Options Country Events properties
[**search_countries**](CountriesApi.md#search_countries) | **POST** /v1/countries | Search Countries



## country_by_ticker

> models::CountryByTicker200Response country_by_ticker(ticker)
Get Country properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of country (format alpha2 or alpha3) | [required] |

### Return type

[**models::CountryByTicker200Response**](CountryByTicker_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## country_indicators_events_by_ticker

> models::CountryIndicatorsEventsByTicker200Response country_indicators_events_by_ticker(ticker)
List Country Events properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of country (format alpha2 or alpha3) | [required] |

### Return type

[**models::CountryIndicatorsEventsByTicker200Response**](CountryIndicatorsEventsByTicker_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_countries

> options_countries()
Options Search Countries

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


## options_country_by_ticker

> options_country_by_ticker(ticker)
Options Country properties

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of country (format alpha2 or alpha3) | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_country_indicators_events_by_ticker

> options_country_indicators_events_by_ticker(ticker)
Options Country Events properties

Options method is used to describe the communication options for the targeted resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** | ticker name of country (format alpha2 or alpha3) | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_countries

> models::SearchCountries200Response search_countries(search_instruments_request)
Search Countries

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_instruments_request** | Option<[**SearchInstrumentsRequest**](SearchInstrumentsRequest.md)> | Some Description |  |

### Return type

[**models::SearchCountries200Response**](SearchCountries_200_response.md)

### Authorization

[cog-p-eu-eod-analysis](../README.md#cog-p-eu-eod-analysis)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

