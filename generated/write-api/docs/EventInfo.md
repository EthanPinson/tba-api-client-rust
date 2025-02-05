# EventInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**first_code** | Option<**String**> | Event code used to sync data with FIRST | [optional]
**playoff_type** | Option<**i32**> | Integer constant representing the playoff format. References constants here: https://github.com/the-blue-alliance/the-blue-alliance/blob/master/consts/playoff_type.py | [optional]
**webcasts** | Option<[**Vec<models::EventInfoWebcastsInner>**](EventInfo_webcasts_inner.md)> | A list of webcast URLs to set for this event. This will overwrite the existing webcast list | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


