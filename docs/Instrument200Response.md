# Instrument200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Id is the unique identifier for the security. | [optional]
**ticker** | Option<**String**> | Ticker is the unique identifier for the security. | [optional]
**ticker_alternative** | Option<**String**> | TickerAlternative is the alternative identifier for the security. | [optional]
**name** | Option<**String**> | Name is the name of the security. | [optional]
**description** | Option<**String**> | Description is the description of the security. | [optional]
**exchange** | Option<[**models::V1ExchangesBulkResponse**](v1ExchangesBulkResponse.md)> |  | [optional]
**asset** | Option<**String**> | Asset is the asset class of the security. | [optional]
**cik** | Option<**String**> | CIK is the Central Index Key (CIK) is a number used to identify the filings of a business. | [optional]
**isin** | Option<**String**> | ISIN is the International Securities Identification Number (ISIN) is a code that uniquely identifies a specific securities issue. | [optional]
**adr** | Option<**bool**> | ADR is a boolean value that indicates if the security is an American Depositary Receipt. | [optional]
**activity** | Option<**std::collections::HashMap<String, String>**> | Activity is a map of activities that the security is involved in. | [optional]
**quote** | Option<[**models::V1QuoteResponse**](v1QuoteResponse.md)> |  | [optional]
**dividends** | Option<[**Vec<models::V1DividendResponse>**](v1DividendResponse.md)> | Dividends is a list of dividends that the security has paid. | [optional]
**stats** | Option<[**models::Instrument200ResponseStats**](Instrument_200_response_stats.md)> |  | [optional]
**employees** | Option<**i32**> | Employees is the number of employees that the company has. | [optional]
**shares** | Option<[**models::Instrument200ResponseShares**](Instrument_200_response_shares.md)> |  | [optional]
**tax** | Option<[**models::Instrument200ResponseTax**](Instrument_200_response_tax.md)> |  | [optional]
**contact** | Option<[**models::Instrument200ResponseContact**](Instrument_200_response_contact.md)> |  | [optional]
**address** | Option<[**models::Instrument200ResponseAddress**](Instrument_200_response_address.md)> |  | [optional]
**created_at** | Option<**i64**> | CreatedAt is the time at which the instrument was created. | [optional]
**updated_at** | Option<**i64**> | UpdatedAt is the time at which the instrument was last updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


