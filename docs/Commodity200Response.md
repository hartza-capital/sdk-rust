# Commodity200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticker** | Option<**String**> | Unique identifier symbol for this financial instrument, following standard market conventions. | [optional]
**name** | Option<**String**> | Name is the name of the commodity. Ex: Gold. | [optional]
**description** | Option<**String**> | Description is the description of the commodity. | [optional]
**activities** | Option<**std::collections::HashMap<String, String>**> | Activities is a map of activities that the commodity is involved in. | [optional]
**quote** | Option<[**models::V1QuoteResponse**](v1QuoteResponse.md)> |  | [optional]
**stats** | Option<[**models::V1StatsResponse**](v1StatsResponse.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


