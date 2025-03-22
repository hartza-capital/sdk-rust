# V1CurrencyResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticker** | Option<**String**> | Ticker is the unique identifier for the currency. ISO 4217 code. | [optional]
**imf_reserve** | Option<**bool**> | IMFReserve is a boolean value that indicates if the currency is a reserve currency of the International Monetary Fund. | [optional]
**central_bank** | Option<[**models::V1CurrencyResponseCentralBank**](v1CurrencyResponse_central_bank.md)> |  | [optional]
**exchanges** | Option<[**Vec<models::V1ExchangesBulkResponse>**](v1ExchangesBulkResponse.md)> | Exchanges is the list of exchanges used in the country. | [optional]
**countries** | Option<[**Vec<models::V1CountryBulkResponse>**](v1CountryBulkResponse.md)> | Countries is the list of countries using the currency. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


