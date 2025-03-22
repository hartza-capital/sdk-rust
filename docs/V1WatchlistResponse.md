# V1WatchlistResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticker** | Option<**String**> | Ticker is the unique identifier for the filter. | [optional]
**description** | Option<**String**> | Description is the description of the filter. | [optional]
**filters** | Option<[**Vec<models::V1ScreenerFilter>**](v1ScreenerFilter.md)> | Filters is a list of filters. | [optional]
**sort** | Option<[**models::V1ScreenerSort**](v1ScreenerSort.md)> |  | [optional]
**results** | Option<[**Vec<models::V1SearchResponse>**](v1SearchResponse.md)> |  | [optional]
**stats** | Option<[**models::V1WatchlistResponseStats**](v1WatchlistResponse_stats.md)> |  | [optional]
**report_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | UUID of report | [optional]
**created_at** | Option<**i64**> | CreatedAt is the creation date of the filter. | [optional]
**updated_at** | Option<**i64**> | UpdatedAt is the last update date of the filter. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


