# V1SearchResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Id is the unique identifier for the instrument. | [optional]
**ticker** | Option<**String**> | Ticker is the unique identifier for the instrument. | [optional]
**ticker_alternative** | Option<**String**> | TickerAlternative is an alternative identifier for the instrument. | [optional]
**exchange** | Option<**String**> | Exchange is the exchange where the instrument is traded. | [optional]
**name** | Option<**String**> | Name is the name of the instrument. | [optional]
**isin** | Option<**String**> | ISIN is the International Securities Identification Number. | [optional]
**cik** | Option<**String**> | CIK is the Central Index Key. Only for US companies. | [optional]
**asset** | Option<**String**> | Asset is the asset class of the instrument. | [optional]
**activity** | Option<**i64**> | Activity is the activity of the instrument. | [optional]
**adr** | Option<**bool**> | ADR is a boolean value that indicates if the instrument is an American Depositary Receipt. | [optional]
**activities** | Option<**Vec<String>**> | Activities is a list of activities that the instrument is involved in. | [optional]
**size** | Option<**String**> | Size is the size of the instrument. | [optional]
**history** | Option<**Vec<f32>**> | History is a list of historical quotes for the instrument. The list is sorted by date in ascending order. | [optional]
**change** | Option<**f64**> | Change is the change in the price of the instrument. | [optional]
**last** | Option<**f64**> | Last is the last price of the instrument. | [optional]
**start** | Option<**f64**> | Start is the start price of the instrument. | [optional]
**max_annual** | Option<**f64**> | MaxAnnual is the maximum annual price of the instrument. | [optional]
**min_annual** | Option<**f64**> | MinAnnual is the minimum annual price of the instrument. | [optional]
**currency** | Option<**String**> | Currency is the currency of the instrument. | [optional]
**country** | Option<**String**> | Country is the country of the instrument. | [optional]
**volume_avg_10d** | Option<**i64**> | VolumeAvg10d is the average volume of the instrument over the last 10 days. | [optional]
**volume_avg_30d** | Option<**i64**> | VolumeAvg30d is the average volume of the instrument over the last 30 days. | [optional]
**volume_avg_90d** | Option<**i64**> | VolumeAvg90d is the average volume of the instrument over the last 90 days. | [optional]
**volume_ratio** | Option<**f64**> | VolumeRatio is the ratio of the volume of the instrument to the average volume over the last 90 days. | [optional]
**market_cap_usd** | Option<**i64**> | MarketCapUSD is the market capitalization of the instrument in USD. | [optional]
**return_year** | Option<**f64**> | ReturnYear is the return of the instrument over the last year. | [optional]
**dividend_yield** | Option<**f64**> | DividendYield is the dividend yield of the instrument. | [optional]
**beta5y** | Option<**f64**> | Beta5y is the beta of the instrument over the last 5 years. | [optional]
**created_at** | Option<**i64**> | CreatedAt is the date and time of the creation of the instrument. | [optional]
**updated_at** | Option<**i64**> | UpdatedAt is the date and time of the last update. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


