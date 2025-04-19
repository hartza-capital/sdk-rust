# Index200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticker** | Option<**String**> | Unique identifier symbol for this financial instrument, following standard market conventions. | [optional]
**name** | Option<**String**> | Name is the name of the security. | [optional]
**exchange** | Option<**String**> | Exchange is the exchange where the security is traded. | [optional]
**country** | Option<**bool**> | Country is a boolean value that indicates if the security is the principal country index. | [optional]
**activity** | Option<**std::collections::HashMap<String, String>**> | Activity is a map of activities that the security is involved in. | [optional]
**sub_indexes** | Option<**Vec<String>**> | SubIndexes is a list of indexes that are part of the security. | [optional]
**sizecap** | Option<**String**> | Size of index | [optional]
**quote** | Option<[**models::V1QuoteResponse**](v1QuoteResponse.md)> |  | [optional]
**stats** | Option<[**models::V1StatsResponse**](v1StatsResponse.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


