# V1StatsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**size** | Option<**String**> |  | [optional]
**market_cap_usd** | Option<**i64**> | MarketCapUSD is the market capitalization of the security in USD. | [optional]
**price** | Option<[**models::StatsResponsePrice**](StatsResponsePrice.md)> |  | [optional]
**volumes** | Option<[**models::StatsResponseVolumes**](StatsResponseVolumes.md)> |  | [optional]
**dividend_yield** | Option<**f64**> | DividendYield is the dividend yield of the security. It is the ratio of a company's annual dividend compared to its share price. | [optional]
**updated_at** | Option<**i64**> | Timestamp (in Unix epoch seconds) when this resource was last modified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


