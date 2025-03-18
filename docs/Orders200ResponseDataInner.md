# Orders200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Id is the unique identifier for the order. | [optional]
**instrument_id** | Option<**i64**> | InstrumentId is the unique identifier for the instrument. | [optional]
**ticker** | Option<**String**> | Ticker is the unique identifier for the instrument. | [optional]
**exchange** | Option<**String**> | Exchange is the exchange where the instrument is traded. | [optional]
**name** | Option<**String**> | Name is the name of the instrument. | [optional]
**asset** | Option<**String**> | Asset is the asset class of the instrument. | [optional]
**action** | Option<**String**> | Action is the action of the order. | [optional]
**order_type** | Option<**String**> | OrderType is the type of the order. | [optional]
**duration** | Option<**String**> | Duration is the duration of the order. | [optional]
**size** | Option<[**models::Orders200ResponseDataInnerSize**](Orders_200_response_data_inner_size.md)> |  | [optional]
**price** | Option<[**models::Orders200ResponseDataInnerPrice**](Orders_200_response_data_inner_price.md)> |  | [optional]
**status** | Option<**String**> | Status is the status of the orders. | [optional]
**updated_at** | Option<**i64**> | UpdatedAt is the date and time of the last update. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


