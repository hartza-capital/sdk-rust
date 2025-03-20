# Country200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticker** | Option<**String**> | Ticker is the unique identifier for the country. | [optional]
**ticker_alternative** | Option<**String**> | TickerAlternative is the alternative unique identifier for the country. | [optional]
**name** | Option<**String**> | Name is the name of the country. It is the human readable name of the country. | [optional]
**languages** | Option<**std::collections::HashMap<String, String>**> | Languages is the list of languages used in the country. Example: French for France. | [optional]
**economic** | Option<[**models::Country200ResponseEconomic**](Country_200_response_economic.md)> |  | [optional]
**telecom** | Option<[**models::Country200ResponseTelecom**](Country_200_response_telecom.md)> |  | [optional]
**geographic** | Option<[**models::Country200ResponseGeographic**](Country_200_response_geographic.md)> |  | [optional]
**indicators** | Option<[**std::collections::HashMap<String, models::Country200ResponseIndicatorsValue>**](Country_200_response_indicators_value.md)> | Indicators is the list of MacroEconomic indicators used in the country. Example: GDP for France. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


