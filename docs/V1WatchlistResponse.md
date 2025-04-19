# V1WatchlistResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticker** | Option<**String**> | Ticker is the unique identifier for the watchlist. | [optional]
**description** | Option<**String**> | Description is the description of the filter. | [optional]
**filters** | Option<[**Vec<models::V1ScreenerFilter>**](v1ScreenerFilter.md)> | List of instruments in the watchlist. | [optional]
**sort** | Option<[**models::V1ScreenerSort**](v1ScreenerSort.md)> |  | [optional]
**results** | Option<[**Vec<models::V1SearchResponse>**](v1SearchResponse.md)> |  | [optional]
**stats** | Option<[**models::V1WatchlistResponseStats**](v1WatchlistResponse_stats.md)> |  | [optional]
**report_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | UUID of report generated for the watchlist. | [optional]
**created_at** | Option<**i64**> | Timestamp (in Unix epoch seconds) when this resource was initially created in the system. | [optional]
**updated_at** | Option<**i64**> | Timestamp (in Unix epoch seconds) when this resource was last modified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


