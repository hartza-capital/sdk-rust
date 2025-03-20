# PatchAccountByIdRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name is a short description of account | [optional]
**description** | Option<**String**> | Description is the description of the account. | [optional]
**currency_base** | Option<**String**> | CurrencyBase is the base currency of the account. It is the currency used to calculate the account value. | [optional]
**strategy** | Option<**String**> | Ref of strategy apply on account. | [optional]
**max_drawdown** | Option<**f64**> | MaxDrawdown is the maximum drawdown of the account. It is the maximum loss from a peak to a trough of a portfolio, before a new peak is attained. | [optional]
**max_positions** | Option<**i32**> | MaxPositions is the maximum number of positions in the account. It is the maximum number of positions that can be held in the account. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


