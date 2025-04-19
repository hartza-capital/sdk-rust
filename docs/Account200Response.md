# Account200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name is a short description of account | [optional]
**description** | Option<**String**> | Description is the description of the account. | [optional]
**currency_base** | Option<**String**> | CurrencyBase is the base currency of the account. It is the currency used to calculate the account value. | [optional]
**watchlist** | Option<[**models::V1WatchlistResponse**](v1WatchlistResponse.md)> |  | [optional]
**strategy** | Option<[**models::V1StrategyResponse**](v1StrategyResponse.md)> |  | [optional]
**max_drawdown** | Option<**f64**> | MaxDrawdown is the maximum drawdown of the account. It is the maximum loss from a peak to a trough of a portfolio, before a new peak is attained. | [optional]
**max_positions** | Option<**i32**> | MaxPositions is the maximum number of positions in the account. It is the maximum number of positions that can be held in the account. | [optional]
**status** | Option<**String**> | Status is the status of the account. It is the status of the account. | [optional]
**created_at** | Option<**i64**> | Timestamp (in Unix epoch seconds) when this resource was initially created in the system. | [optional]
**updated_at** | Option<**i64**> | Timestamp (in Unix epoch seconds) when this resource was last modified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


